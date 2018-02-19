use std::io::{Read, Error, ErrorKind};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::str;
use rand;
use rand::Rng;
use sha2::{Sha512, Digest};
use hyper::server::Response;
use hyper::{self, Uri};
use futures::{future, Future};
use srp::server::{UserRecord, SrpServer};
use srp::client::{SrpClient, srp_private_key};
use srp::groups::G_3072;
use srp::types::SrpGroup;
use num::BigUint;
use std::ops::BitXor;
use ring::{hkdf, hmac, digest};
use chacha20_poly1305_aead;
use crypto::ed25519;
use uuid::Uuid;

use accessory::HapAccessory;

use db::storage::Storage;
use db::database::Database;
use config::Config;
use transport::http::tlv_response;
use transport::http::handlers::Handler;
use transport::tlv;
use protocol::device::Device;
use protocol::pairing::Pairing;

struct Session {
    salt: Vec<u8>,
    verifier: Vec<u8>,
    b: Vec<u8>,
    b_pub: Vec<u8>,
    shared_secret: Option<Vec<u8>>,
}

pub struct PairSetup {
    session: Option<Session>
}

impl PairSetup {
    pub fn new() -> PairSetup {
        PairSetup { session: None }
    }
}

impl<S: Storage> Handler<S> for PairSetup {
    fn handle(&mut self, _: Uri, body: Vec<u8>, database: &Arc<Mutex<Database<S>>>, accessories: &Arc<Vec<Box<HapAccessory>>>) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let decoded = tlv::decode(body);
        let mut answer: HashMap<u8, Vec<u8>> = HashMap::new();

        if let Some(v) = decoded.get(&0x06) {
            match v[0] {
                1 => {
                    println!("/pair-setup - M1: Got SRP Start Request");

                    let (t, v) = tlv::Type::State(2).as_type_value();
                    answer.insert(t, v);

                    // TODO - Errors for kTLVError_Unavailable, kTLVError_MaxTries and kTLVError_Busy

                    let accessory = Device::load::<S>(database).unwrap();

                    let mut rng = rand::thread_rng();
                    let salt = rng.gen_iter::<u8>().take(16).collect::<Vec<u8>>(); // s
                    let b = rng.gen_iter::<u8>().take(64).collect::<Vec<u8>>();

                    let private_key = srp_private_key::<Sha512>(b"Pair-Setup", accessory.pin.as_bytes(), &salt); // x = H(s | H(I | ":" | P))
                    let srp_client = SrpClient::<Sha512>::new(&private_key, &G_3072);
                    let verifier = srp_client.get_password_verifier(&private_key); // v = g^x

                    let user = UserRecord {
                        username: b"Pair-Setup",
                        salt: &salt,
                        verifier: &verifier,
                    };
                    // TODO - return a kTLVError
                    let srp_server = SrpServer::<Sha512>::new(&user, b"foo", &b, &G_3072).unwrap();
                    let b_pub = srp_server.get_b_pub();

                    self.session = Some(Session {
                        salt: salt.clone(),
                        verifier: verifier.clone(),
                        b: b.clone(),
                        b_pub: b_pub.clone(),
                        shared_secret: None,
                    });

                    let (t, v) = tlv::Type::PublicKey(b_pub).as_type_value();
                    answer.insert(t, v);
                    let (t, v) = tlv::Type::Salt(salt.clone()).as_type_value();
                    answer.insert(t, v);

                    println!("/pair-setup - M2: Sending SRP Start Response");
                },
                3 => {
                    println!("/pair-setup - M3: Got SRP Verify Request");

                    let (t, v) = tlv::Type::State(4).as_type_value();
                    answer.insert(t, v);

                    if let Some(ref mut session) = self.session {
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
                                return Box::new(future::ok(tlv_response(answer)));
                            },
                            Ok(b_proof) => {
                                let (t, v) = tlv::Type::Proof(b_proof).as_type_value();
                                answer.insert(t, v);
                            },
                        }

                        session.shared_secret = Some(shared_secret.as_slice().to_vec());

                        println!("/pair-setup - M4: Sending SRP Verify Response");
                    } else {
                        // some error
                    }
                },
                5 => {
                    println!("/pair-setup - M5: Got SRP Exchange Request");

                    let (t, v) = tlv::Type::State(6).as_type_value();
                    answer.insert(t, v);

                    if let Some(ref mut session) = self.session {
                        if let Some(ref mut shared_secret) = session.shared_secret {
                            let data = decoded.get(&0x05).unwrap();
                            let encrypted_data = Vec::from(&data[..data.len() - 16]);
                            let auth_tag = Vec::from(&data[data.len() - 16..]);

                            let mut encryption_key = [0; 32];
                            let salt = hmac::SigningKey::new(&digest::SHA512, b"Pair-Setup-Encrypt-Salt");
                            hkdf::extract_and_expand(&salt, &shared_secret, b"Pair-Setup-Encrypt-Info", &mut encryption_key);

                            let mut decrypted_data = Vec::new();
                            let mut nonce = vec![0; 4];
                            nonce.extend(b"PS-Msg05");
                            chacha20_poly1305_aead::decrypt(&encryption_key, &nonce, &[], &encrypted_data, &auth_tag, &mut decrypted_data).unwrap();

                            let sub_tlv = tlv::decode(decrypted_data);
                            let device_pairing_id = sub_tlv.get(&0x01).unwrap();
                            let device_ltpk = sub_tlv.get(&0x03).unwrap();
                            let device_signature = sub_tlv.get(&0x0A).unwrap();

                            let mut device_x = [0; 32];
                            let salt = hmac::SigningKey::new(&digest::SHA512, b"Pair-Setup-Controller-Sign-Salt");
                            hkdf::extract_and_expand(&salt, &shared_secret, b"Pair-Setup-Controller-Sign-Info", &mut device_x);

                            let mut device_info: Vec<u8> = Vec::new();
                            device_info.extend(&device_x);
                            device_info.extend(device_pairing_id);
                            device_info.extend(device_ltpk);
                            if !ed25519::verify(&device_info, &device_ltpk, &device_signature) {
                                let (t, v) = tlv::Type::Error(tlv::ErrorKind::Authentication).as_type_value();
                                answer.insert(t, v);
                                return Box::new(future::ok(tlv_response(answer)));
                            }

                            // TODO - kTLVError_MaxPeers

                            let uuid_str = str::from_utf8(device_pairing_id).unwrap();
                            let pairing_uuid = Uuid::parse_str(uuid_str).unwrap();
                            let mut pairing_ltpk = [0; 32];
                            for i in 0..32 {
                                pairing_ltpk[i] = device_ltpk[i];
                            }
                            let pairing = Pairing::new(pairing_uuid, pairing_ltpk);
                            pairing.save(database).unwrap();

                            let mut accessory_x = [0; 32];
                            let salt = hmac::SigningKey::new(&digest::SHA512, b"Pair-Setup-Accessory-Sign-Salt");
                            hkdf::extract_and_expand(&salt, &shared_secret, b"Pair-Setup-Accessory-Sign-Info", &mut accessory_x);

                            let accessory = Device::load::<S>(database).unwrap();
                            let mut accessory_info: Vec<u8> = Vec::new();
                            accessory_info.extend(&accessory_x);
                            accessory_info.extend(accessory.id.as_bytes());
                            accessory_info.extend(&accessory.public_key);
                            let accessory_signature = ed25519::signature(&accessory_info, &accessory.private_key);

                            let mut sub_tlv: HashMap<u8, Vec<u8>> = HashMap::new();
                            let (t, v) = tlv::Type::Identifier(accessory.id).as_type_value();
                            sub_tlv.insert(t, v);
                            let (t, v) = tlv::Type::PublicKey(accessory.public_key.to_vec()).as_type_value();
                            sub_tlv.insert(t, v);
                            let (t, v) = tlv::Type::Signature(accessory_signature.to_vec()).as_type_value();
                            sub_tlv.insert(t, v);
                            let encoded_sub_tlv = tlv::encode(sub_tlv);

                            let mut encrypted_data = Vec::new();
                            let mut nonce = vec![0; 4];
                            nonce.extend(b"PS-Msg06");
                            let auth_tag = chacha20_poly1305_aead::encrypt(&encryption_key, &nonce, &[], &encoded_sub_tlv, &mut encrypted_data).unwrap();
                            encrypted_data.extend(&auth_tag);

                            let (t, v) = tlv::Type::EncryptedData(encrypted_data).as_type_value();
                            answer.insert(t, v);

                            println!("/pair-setup - M6: Sending SRP Exchange Response");
                        } else {
                            // some error or just nothing?
                        }
                    } else {
                        // some error or just nothing?
                    }
                },
                _ => {
                    println!("/pair-setup - M{}: Got invalid state", v[0]);
                    let (t, v) = tlv::Type::State(0).as_type_value();
                    answer.insert(t, v);
                    // TODO - return a kTLVError?
                },
            }
        } else {
            let (t, v) = tlv::Type::State(0).as_type_value();
            answer.insert(t, v);
        }

        Box::new(future::ok(tlv_response(answer)))
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

    if a_proof.clone() == d.result().as_slice() {
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
