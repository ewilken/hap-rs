use std::io::{Read, Error, ErrorKind};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::IpAddr;
use rand;
use rand::Rng;
use sha2::{Sha512, Digest};
use iron::prelude::{Request, Response, IronResult};
use iron::status;
use srp::server::{UserRecord, SrpServer};
use srp::client::{SrpClient, srp_private_key};
use srp::groups::G_3072;
use srp::types::SrpGroup;
use serde_json;
use num::BigUint;
use std::ops::{BitXor, Mul};
use crypto::chacha20poly1305::ChaCha20Poly1305;
use crypto::aead::AeadDecryptor;

use db::context::Context;
use config::Config;
use transport::http::ContentType;
use transport::tlv;

pub fn pair_setup(request: &mut Request, config: &Arc<Config>, context: &Arc<Mutex<Context>>) -> IronResult<Response> {
    let ip = Context::get_request_address(request).ip();

    let mut buf: Vec<u8> = Vec::new();
    request.body.by_ref().read_to_end(&mut buf).unwrap();

    let decoded = tlv::decode(buf);
    let mut answer: HashMap<u8, Vec<u8>> = HashMap::new();

    if let Some(v) = decoded.get(&0x06) {
        match v[0] {
            1 => {
                println!("/pair-setup - Got M1: SRP Start Request from {}", ip);

                let (t, v) = tlv::Type::State(2).as_type_value();
                answer.insert(t, v);

                // TODO - Errors for kTLVError_Unavailable, kTLVError_MaxTries and kTLVError_Busy

                let mut rng = rand::thread_rng();
                let salt = rng.gen_iter::<u8>().take(16).collect::<Vec<u8>>(); // s
                let b = rng.gen_iter::<u8>().take(64).collect::<Vec<u8>>();

                let private_key = srp_private_key::<Sha512>(b"Pair-Setup", config.pin.as_bytes(), &salt); // x = H(s | H(I | ":" | P))
                let srp_client = SrpClient::<Sha512>::new(&private_key, &G_3072);
                let verifier = srp_client.get_password_verifier(&private_key); // v = g^x

                let user = UserRecord {
                    username: b"Pair-Setup",
                    salt: &salt,
                    verifier: &verifier,
                };
                // TODO - return a kTLVError
                let srp_server = SrpServer::<Sha512>::new(&user, b"foo", &b, &G_3072).unwrap();

                let session = SrpPairingSession {
                    ip,
                    salt: salt.to_owned(),
                    verifier: verifier.to_owned(),
                    b: b.to_owned(),
                    b_pub: srp_server.get_b_pub(),
                    a_pub: None,
                    next_step: 3,
                };

                let (t, v) = tlv::Type::PublicKey(session.b_pub.to_owned()).as_type_value();
                answer.insert(t, v);
                let (t, v) = tlv::Type::Salt(salt.to_owned()).as_type_value();
                answer.insert(t, v);

                // TODO - get rid of all those unwraps
                session.save(context).unwrap();

                println!("/pair-setup - Sending M2: SRP Start Response to {}", ip);
            },
            3 => {
                println!("/pair-setup - Got M3: SRP Verify Request from {}", ip);

                let (t, v) = tlv::Type::State(2).as_type_value();
                answer.insert(t, v);
                if let Some(mut session) = SrpPairingSession::load(ip, context) {
                    let a_pub = decoded.get(&0x03).unwrap();
                    let a_proof = decoded.get(&0x04).unwrap();

                    let user = UserRecord {
                        username: b"Pair-Setup",
                        salt: &session.salt,
                        verifier: &session.verifier,
                    };
                    let srp_server = SrpServer::<Sha512>::new(&user, &a_pub, &session.b, &G_3072).unwrap();
                    let shared_secret = srp_server.get_key();
                    let b_proof = verify_client_proof::<Sha512>(&session.b_pub, &a_pub, &a_proof, &session.salt, &shared_secret.as_slice().to_vec(), &G_3072).unwrap();

                    let (t, v) = tlv::Type::State(4).as_type_value();
                    answer.insert(t, v);
                    let (t, v) = tlv::Type::Proof(b_proof).as_type_value();
                    answer.insert(t, v);

                    session.a_pub = Some(a_pub.to_owned());
                    session.save(context).unwrap();

                    println!("/pair-setup - Sending M4: SRP Verify Response to {}", ip);
                } else {
                    // some error
                }
            },
            5 => {
                println!("/pair-setup - Got M5: SRP Exchange Request from {}", ip);

                let data = decoded.get(&0x05).unwrap();
                let encrypted_data = Vec::from(&data[..data.len() - 16]);
                let auth_tag = Vec::from(&data[data.len() - 16..]);

                if let Some(session) = SrpPairingSession::load(ip, context) {
                    if let Some(a_pub) = session.a_pub {
                        let session_key = compute_session_key::<Sha512>(&a_pub, &session.b_pub, &session.b, &session.verifier);

                        let mut encryption_key = Sha512::new();
                        encryption_key.input(&session_key);
                        encryption_key.input(b"Pair-Setup-Encrypt-Salt");
                        encryption_key.input(b"Pair-Setup-Encrypt-Info");

                        println!("{:?}", &encryption_key.result()[..32]);

                        let mut chacha_poly = ChaCha20Poly1305::new(&encryption_key.result()[..32], b"PS-Msg05", &[]);
                        let mut decrypted_data = vec![0; encrypted_data.len()];
                        let decryption_successful = chacha_poly.decrypt(&encrypted_data, &mut decrypted_data, &auth_tag);
                        println!("{:?}", decrypted_data);
                        println!("{:?}", decryption_successful);
                    } else {
                        // some error
                    }
                } else {
                    // some error
                }
            },
            _ => {
                println!("/pair-setup - Got invalid state: M{} from {}", v[0], ip);
                let (t, v) = tlv::Type::State(0).as_type_value();
                answer.insert(t, v);
                // TODO - return a kTLVError?
            },
        }
    } else {
        let (t, v) = tlv::Type::State(0).as_type_value();
        answer.insert(t, v);
    }

    let body = tlv::encode(answer);

    let mut response = Response::with((status::Ok, body));
    response.headers.set_raw("Content-Type", vec![ContentType::PairingTLV8.as_vec()]);

    Ok(response)
}

#[derive(Serialize, Deserialize)]
struct SrpPairingSession {
    ip: IpAddr,
    salt: Vec<u8>,
    verifier: Vec<u8>,
    b: Vec<u8>,
    b_pub: Vec<u8>,
    a_pub: Option<Vec<u8>>,
    next_step: u8,
}

impl SrpPairingSession {
    fn load(ip: IpAddr, context: &Arc<Mutex<Context>>) -> Option<SrpPairingSession> {
        let key = match ip {
            IpAddr::V4(addr) => addr.octets().to_vec(),
            IpAddr::V6(addr) => addr.octets().to_vec(),
        };
        let c = context.lock().unwrap();
        if let Some(val) = c.get(key) {
            return serde_json::from_slice(&val).ok();
        }
        None
    }

    fn save(&self, context: &Arc<Mutex<Context>>) -> Result<(), Error> {
        let key = match self.ip {
            IpAddr::V4(addr) => addr.octets().to_vec(),
            IpAddr::V6(addr) => addr.octets().to_vec(),
        };
        let val = serde_json::to_vec(self)?;
        let mut c = context.lock().unwrap();
        c.set(key, val);
        Ok(())
    }
}

// TODO - fix the actual srp package to do proper verification
fn verify_client_proof<D: Digest>(b_pub: &Vec<u8>, a_pub: &Vec<u8>, a_proof: &Vec<u8>, salt: &Vec<u8>, key: &Vec<u8>, group: &SrpGroup) -> Result<Vec<u8>, Error> {
    let mut dhn = D::new();
    dhn.input(&group.n.to_bytes_be());
    let hn = BigUint::from_bytes_be(&dhn.result());

    let mut dhg = D::new();
    dhg.input(&group.g.to_bytes_be());
    let hg = BigUint::from_bytes_be(&dhg.result());

    let hng = hn.bitxor(hg);

    let mut dhi = D::new();
    dhi.input(b"Pair-Setup");
    let hi = dhi.result();

    let mut d = D::new();
    //M = H(H(N) xor H(g), H(I), s, A, B, K)
    d.input(&hng.to_bytes_be());
    d.input(&hi);
    d.input(salt);
    d.input(a_pub);
    d.input(b_pub);
    d.input(key);

    if a_proof.to_owned() == d.result().as_slice() {
        // H(A, M, K)
        let mut d = D::new();
        d.input(a_pub);
        d.input(a_proof);
        d.input(key);
        Ok(d.result().as_slice().to_vec())
    } else {
        Err(Error::new(ErrorKind::Other, "invalid user proof"))
    }
}

fn compute_session_key<D: Digest>(a_pub: &Vec<u8>, b_pub: &Vec<u8>, b: &Vec<u8>, verifier: &Vec<u8>) -> Vec<u8> {
    let v = BigUint::from_bytes_be(verifier);

    // u = H(A, B)
    let mut hu = D::new();
    hu.input(&pad(a_pub));
    hu.input(&pad(b_pub));
    let u = BigUint::from_bytes_be(&hu.result().as_slice().to_vec());

    // S = (Av^u) mod N
    let A = BigUint::from_bytes_be(a_pub);
    let mut S = powm(&v, &u, &G_3072.n);
    S = S.mul(A);
    S = S % &G_3072.n;

    // TODO - rejections

    let b = BigUint::from_bytes_be(b);
    S = powm(&S, &b, &G_3072.n);

    let mut K = D::new();
    K.input(&S.to_bytes_be());
    K.result().as_slice().to_vec()
}

fn pad(n: &Vec<u8>) -> Vec<u8> {
    let mut n = n.to_owned();
    let n_len = n.len();
    println!("{:?}", n_len);
    let len = G_3072.n.to_bytes_be().len();
    println!("{:?}", len);
    if n_len < len {
        let mut padding = Vec::from(&G_3072.n.to_bytes_be()[n_len..len]);
        n.append(&mut padding);
    }
    println!("{:?}", n_len);

    n
}

fn powm(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    let zero = BigUint::new(vec![0]);
    let one = BigUint::new(vec![1]);
    let two = BigUint::new(vec![2]);
    let mut exp = exp.clone();
    let mut result = one.clone();
    let mut base = base % modulus;

    while exp > zero {
        if &exp % &two == one {
            result = (result * &base) % modulus;
        }
        exp = exp >> 1;
        base = (&base * &base) % modulus;
    }
    result
}
