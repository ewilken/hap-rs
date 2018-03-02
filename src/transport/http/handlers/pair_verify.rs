use std::io::{Error, ErrorKind};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::str;
use hyper;
use hyper::server::Response;
use hyper::{Uri, StatusCode};
use futures::{future, Future};
use rand::{self, Rng};
use crypto::{curve25519, ed25519};
use ring::{hkdf, hmac, digest};
use chacha20_poly1305_aead;
use uuid::Uuid;
use futures::sync::oneshot;

use transport::http::tlv_response;
use transport::http::handlers::Handler;
use transport::http::encrypted_stream;
use transport::tlv::{self, Type, Value};
use db::accessory_list::AccessoryList;
use db::storage::Storage;
use db::database::Database;
use protocol::device::Device;
use protocol::pairing::Pairing;

struct Session {
    b_pub: [u8; 32],
    a_pub: Vec<u8>,
    shared_secret: [u8; 32],
    session_key: [u8; 32],
}

pub struct PairVerify {
    session: Option<Session>,
    session_sender: Option<oneshot::Sender<encrypted_stream::Session>>,
}

impl PairVerify {
    pub fn new(session_sender: oneshot::Sender<encrypted_stream::Session>) -> PairVerify {
        PairVerify { session: None, session_sender: Some(session_sender) }
    }
}

enum Step {
    Start(Vec<u8>),
    Finish(Vec<u8>),
}

impl Step {
    fn parse(tlv_body: Vec<u8>) -> Result<Step, Error> {
        let decoded = tlv::decode(tlv_body);
        match decoded.get(&(Type::State as u8)) {
            Some(method) => match method[0] {
                1 => {
                    let a_pub = decoded.get(&(Type::PublicKey as u8)).unwrap();
                    Ok(Step::Start(a_pub.clone()))
                },
                3 => {
                    let data = decoded.get(&(Type::EncryptedData as u8)).unwrap();
                    Ok(Step::Finish(data.clone()))
                },
                _ => Err(Error::new(ErrorKind::Other, "invalid method"))
            },
            None => Err(Error::new(ErrorKind::Other, "missing method")),
        }
    }
}

impl<S: Storage> Handler<S> for PairVerify {
    fn handle(&mut self, _: Uri, body: Vec<u8>, _: Arc<Option<Uuid>>, database: &Arc<Mutex<Database<S>>>, _: &AccessoryList) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let mut answer: HashMap<u8, Vec<u8>> = HashMap::new();

        match Step::parse(body) {
            Ok(Step::Start(a_pub)) => {
                debug!("/pair-verify - M1: Got Verify Start Request");

                let (t, v) = Value::State(2).as_tlv();
                answer.insert(t, v);

                let mut rng = rand::thread_rng();
                let b = rng.gen::<[u8; 32]>();
                let b_pub = curve25519::curve25519_base(&b);
                let shared_secret = curve25519::curve25519(&b, &a_pub);

                let accessory = Device::load::<S>(database).unwrap();
                let mut accessory_info: Vec<u8> = Vec::new();
                accessory_info.extend(&b_pub);
                accessory_info.extend(accessory.id.as_bytes());
                accessory_info.extend(&a_pub);
                let accessory_signature = ed25519::signature(&accessory_info, &accessory.private_key);

                let mut sub_tlv: HashMap<u8, Vec<u8>> = HashMap::new();
                let (t, v) = Value::Identifier(accessory.id).as_tlv();
                sub_tlv.insert(t, v);
                let (t, v) = Value::Signature(accessory_signature.to_vec()).as_tlv();
                sub_tlv.insert(t, v);
                let encoded_sub_tlv = tlv::encode(sub_tlv);

                let mut session_key = [0; 32];
                let salt = hmac::SigningKey::new(&digest::SHA512, b"Pair-Verify-Encrypt-Salt");
                hkdf::extract_and_expand(&salt, &shared_secret, b"Pair-Verify-Encrypt-Info", &mut session_key);

                self.session = Some(Session {
                    b_pub,
                    a_pub,
                    shared_secret,
                    session_key,
                });

                let mut encrypted_data = Vec::new();
                let mut nonce = vec![0; 4];
                nonce.extend(b"PV-Msg02");
                let auth_tag = chacha20_poly1305_aead::encrypt(&session_key, &nonce, &[], &encoded_sub_tlv, &mut encrypted_data).unwrap();
                encrypted_data.extend(&auth_tag);

                let (t, v) = Value::PublicKey(b_pub.to_vec()).as_tlv();
                answer.insert(t, v);
                let (t, v) = Value::EncryptedData(encrypted_data).as_tlv();
                answer.insert(t, v);

                debug!("/pair-verify - M2: Sending Verify Start Response");
            },
            Ok(Step::Finish(data)) => {
                debug!("/pair-verify - M3: Got Verify Finish Request");

                let (t, v) = Value::State(4).as_tlv();
                answer.insert(t, v);

                if let Some(ref mut session) = self.session {
                    let encrypted_data = Vec::from(&data[..data.len() - 16]);
                    let auth_tag = Vec::from(&data[data.len() - 16..]);

                    let mut decrypted_data = Vec::new();
                    let mut nonce = vec![0; 4];
                    nonce.extend(b"PV-Msg03");
                    chacha20_poly1305_aead::decrypt(&session.session_key, &nonce, &[], &encrypted_data, &auth_tag, &mut decrypted_data).unwrap();

                    let sub_tlv = tlv::decode(decrypted_data);
                    let device_pairing_id = sub_tlv.get(&(Type::Identifier as u8)).unwrap();
                    let device_signature = sub_tlv.get(&(Type::Signature as u8)).unwrap();

                    let uuid_str = str::from_utf8(device_pairing_id).unwrap();
                    let pairing_uuid = Uuid::parse_str(uuid_str).unwrap();
                    let pairing = Pairing::load::<S>(pairing_uuid, database).unwrap();

                    let mut device_info: Vec<u8> = Vec::new();
                    device_info.extend(&session.a_pub);
                    device_info.extend(device_pairing_id);
                    device_info.extend(&session.b_pub);
                    if !ed25519::verify(&device_info, &pairing.public_key, &device_signature) {
                        let (t, v) = Value::Error(tlv::ErrorKind::Authentication).as_tlv();
                        answer.insert(t, v);
                    }

                    if let Some(sender) = self.session_sender.take() {
                        let encrypted_session = encrypted_stream::Session {
                            controller_id: pairing_uuid,
                            shared_secret: session.shared_secret,
                        };
                        sender.send(encrypted_session);
                    }

                    debug!("/pair-verify - M4: Sending Verify Finish Response");
                }
            },
            Err(_) => {
                // tlv error
            },
        }

        Box::new(future::ok(tlv_response(answer, StatusCode::Ok)))
    }
}
