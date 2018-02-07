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
use std::ops::BitXor;
use ring::{hkdf, hmac, digest};
use chacha20_poly1305_aead;
use crypto::ed25519;
use uuid::Uuid;

use db::context::Context;
use db::storage::Storage;
use db::database::Database;
use config::Config;
use transport::http::ContentType;
use transport::tlv;
use protocol::device::Device;
use protocol::pairing::Pairing;

use std::str;

pub fn pair_setup<D: Storage + Send>(request: &mut Request, config: &Arc<Config>, context: &Arc<Mutex<Context>>, database: &Arc<Mutex<Database<D>>>) -> IronResult<Response> {
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

                let device = Device::load::<D>(context, database).unwrap();

                let private_key = srp_private_key::<Sha512>(b"Pair-Setup", device.pin.as_bytes(), &salt); // x = H(s | H(I | ":" | P))
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
                    shared_secret: None,
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

                let (t, v) = tlv::Type::State(4).as_type_value();
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
                    let b_proof = verify_client_proof::<Sha512>(&session.b_pub, &a_pub, &a_proof, &session.salt, &shared_secret.as_slice().to_vec(), &G_3072);

                    match b_proof {
                        Err(_) => {
                            let (t, v) = tlv::Type::Error(tlv::ErrorKind::Authentication).as_type_value();
                            answer.insert(t, v);
                            return Ok(response(answer));
                        },
                        Ok(b_proof) => {
                            let (t, v) = tlv::Type::Proof(b_proof).as_type_value();
                            answer.insert(t, v);
                        },
                    }

                    session.shared_secret = Some(shared_secret.as_slice().to_vec());
                    session.next_step = 5;
                    session.save(context).unwrap();

                    println!("/pair-setup - Sending M4: SRP Verify Response to {}", ip);
                } else {
                    // some error
                }
            },
            5 => {
                println!("/pair-setup - Got M5: SRP Exchange Request from {}", ip);

                let (t, v) = tlv::Type::State(6).as_type_value();
                answer.insert(t, v);

                let data = decoded.get(&0x05).unwrap();
                let encrypted_data = Vec::from(&data[..data.len() - 16]);
                let auth_tag = Vec::from(&data[data.len() - 16..]);

                if let Some(session) = SrpPairingSession::load(ip, context) {
                    if let Some(shared_secret) = session.shared_secret {
                        let mut encryption_key = vec![0; 32];
                        let salt = hmac::SigningKey::new(&digest::SHA512, b"Pair-Setup-Encrypt-Salt");
                        hkdf::extract_and_expand(&salt, &shared_secret, b"Pair-Setup-Encrypt-Info", &mut encryption_key);

                        let mut decrypted_data = Vec::new();
                        let mut nonce = vec![0, 0, 0, 0];
                        nonce.extend(b"PS-Msg05");
                        chacha20_poly1305_aead::decrypt(&encryption_key, &nonce, &[], &encrypted_data, &auth_tag, &mut decrypted_data).unwrap();

                        let sub_tlv = tlv::decode(decrypted_data);
                        let device_pairing_id = sub_tlv.get(&0x01).unwrap();
                        let device_ltpk = sub_tlv.get(&0x03).unwrap();
                        let device_signature = sub_tlv.get(&0x0A).unwrap();

                        let salt = hmac::SigningKey::new(&digest::SHA512, b"Pair-Setup-Controller-Sign-Salt");
                        hkdf::extract_and_expand(&salt, &shared_secret, b"Pair-Setup-Controller-Sign-Info", &mut encryption_key);

                        let mut device_info: Vec<u8> = Vec::new();
                        device_info.extend(encryption_key.to_owned());
                        device_info.extend(device_pairing_id);
                        device_info.extend(device_ltpk);
                        if !ed25519::verify(&device_info, &device_ltpk, &device_signature) {
                            let (t, v) = tlv::Type::Error(tlv::ErrorKind::Authentication).as_type_value();
                            answer.insert(t, v);
                            return Ok(response(answer));
                        }

                        // TODO - kTLVError_MaxPeers

                        let uuid_str = str::from_utf8(device_pairing_id).unwrap();
                        let pairing_uuid = Uuid::parse_str(uuid_str).unwrap();
                        let mut pairing_ltpk = [0; 32];
                        for i in 0..32 {
                            pairing_ltpk[i] = device_ltpk[i];
                        }
                        let pairing = Pairing::new(pairing_uuid, pairing_ltpk);
                        pairing.save(context, database).unwrap();

                        let accessory = Device::load::<D>(context, database).unwrap();
                        let mut accessory_info: Vec<u8> = Vec::new();
                        accessory_info.extend(&encryption_key);
                        accessory_info.extend(accessory.id.as_bytes());
                        accessory_info.extend(&accessory.public_key);
                        let accessory_signature = ed25519::signature(&accessory_info, &accessory.private_key);

                        let mut sub_tlv: HashMap<u8, Vec<u8>> = HashMap::new();
                        let (t, v) = tlv::Type::Identifier(accessory.id.hyphenated().to_string()).as_type_value();
                        sub_tlv.insert(t, v);
                        let (t, v) = tlv::Type::PublicKey(accessory.public_key.to_vec()).as_type_value();
                        sub_tlv.insert(t, v);
                        let (t, v) = tlv::Type::Signature(accessory_signature.to_vec()).as_type_value();
                        sub_tlv.insert(t, v);
                        let encoded_sub_tlv = tlv::encode(sub_tlv);

                        let mut encrypted_data = Vec::new();
                        let mut nonce = vec![0, 0, 0, 0];
                        nonce.extend(b"PS-Msg06");
                        chacha20_poly1305_aead::encrypt(&encryption_key, &nonce, &[], &encoded_sub_tlv, &mut encrypted_data).unwrap();

                        let (t, v) = tlv::Type::EncryptedData(encrypted_data).as_type_value();
                        answer.insert(t, v);

                        println!("/pair-setup - Sending M6: SRP Exchange Response to {}", ip);
                    } else {
                        // some error or just nothing?
                    }
                } else {
                    // some error or just nothing?
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

    Ok(response(answer))
}

fn response(answer: HashMap<u8, Vec<u8>>) -> Response {
    let body = tlv::encode(answer);
    let mut response = Response::with((status::Ok, body));
    response.headers.set_raw("Content-Type", vec![ContentType::PairingTLV8.as_vec()]);
    response
}

#[derive(Serialize, Deserialize)]
struct SrpPairingSession {
    ip: IpAddr,
    salt: Vec<u8>,
    verifier: Vec<u8>,
    b: Vec<u8>,
    b_pub: Vec<u8>,
    shared_secret: Option<Vec<u8>>,
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
