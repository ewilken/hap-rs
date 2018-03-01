use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::str;
use hyper::server::Response;
use hyper::{self, Uri, StatusCode};
use futures::{future, Future};
use uuid::Uuid;

use accessory::HapAccessory;

use db::storage::Storage;
use db::database::Database;
use config::Config;
use transport::http::tlv_response;
use transport::http::handlers::Handler;
use transport::tlv::{self, ErrorKind};
use db::accessory_list::AccessoryList;
use protocol::device::Device;
use protocol::pairing::{Pairing, Permissions};

struct Session {}

pub struct Pairings {
    session: Option<Session>
}

impl Pairings {
    pub fn new() -> Pairings {
        Pairings { session: None }
    }
}

impl<S: Storage> Handler<S> for Pairings {
    fn handle(&mut self, _: Uri, body: Vec<u8>, database: &Arc<Mutex<Database<S>>>, _: &AccessoryList) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let decoded = tlv::decode(body);
        let mut answer: HashMap<u8, Vec<u8>> = HashMap::new();

        if let (Some(v), Some(m)) = (decoded.get(&0x06), decoded.get(&0x00)) {
            match (v[0], m[0]) {
                (1, 3) => {
                    debug!("/pairings - M1: Got Add Pairing Request");

                    let (t, v) = tlv::Type::State(2).as_type_value();
                    answer.insert(t, v);

                    let pairing_id = decoded.get(&0x01).unwrap();
                    let ltpk = decoded.get(&0x03).unwrap();
                    let permissions = Permissions::from_u8(decoded.get(&0x0B).unwrap()[0]).unwrap();
                    let uuid_str = str::from_utf8(pairing_id).unwrap();
                    let pairing_uuid = Uuid::parse_str(uuid_str).unwrap();

                    // TODO - check if controller is admin

                    let d = database.lock().unwrap();
                    match d.get_pairing(pairing_uuid) {
                        Ok(mut pairing) => {
                            if &pairing.public_key.to_vec() != ltpk {
                                let (t, v) = tlv::Type::Error(ErrorKind::Unknown).as_type_value();
                                answer.insert(t, v);
                            }
                            pairing.permissions = permissions;
                            d.set_pairing(&pairing).unwrap();
                        },
                        Err(_) => {
                            // TODO - either check max_peers or just ignore it
                            let mut public_key = [0; 32];
                            public_key.clone_from_slice(&ltpk);
                            let pairing = Pairing {id: pairing_uuid, permissions, public_key};
                            if d.set_pairing(&pairing).is_err() {
                                let (t, v) = tlv::Type::Error(ErrorKind::Unknown).as_type_value();
                                answer.insert(t, v);
                            }
                        },
                    }

                    debug!("/pairings - M2: Sending Add Pairing Response");
                },
                (1, 4) => {
                    debug!("/pairings - M1: Got Remove Pairing Request");

                    let (t, v) = tlv::Type::State(2).as_type_value();
                    answer.insert(t, v);

                    let pairing_id = decoded.get(&0x01).unwrap();
                    let uuid_str = str::from_utf8(pairing_id).unwrap();
                    let pairing_uuid = Uuid::parse_str(uuid_str).unwrap();
                    let d = database.lock().unwrap();
                    match d.get_pairing(pairing_uuid) {
                        Ok(pairing) => {
                            if pairing.permissions != Permissions::Admin {
                                let (t, v) = tlv::Type::Error(ErrorKind::Authentication).as_type_value();
                                answer.insert(t, v);
                                return Box::new(future::ok(tlv_response(answer, StatusCode::Ok)));
                            }
                            if d.delete_pairing(&pairing.id).is_err() {
                                let (t, v) = tlv::Type::Error(ErrorKind::Unknown).as_type_value();
                                answer.insert(t, v);
                            }
                        },
                        Err(_) => {},
                    }

                    debug!("/pairings - M2: Sending Remove Pairing Response");
                },
                (1, 5) => {
                    debug!("/pairings - M1: Got List Pairings Request");

                    let (t, v) = tlv::Type::State(2).as_type_value();
                    answer.insert(t, v);

                    // TODO - check if controller is admin

                    let d = database.lock().unwrap();
                    let pairings = d.list_pairings().unwrap();
                    for (i, pairing) in pairings.iter().enumerate() {
                        let (t, v) = tlv::Type::Identifier(pairing.id.hyphenated().to_string()).as_type_value();
                        answer.insert(t, v);
                        let (t, v) = tlv::Type::PublicKey(pairing.public_key.to_vec()).as_type_value();
                        answer.insert(t, v);
                        let (t, v) = tlv::Type::Permissions(pairing.permissions.clone()).as_type_value();
                        answer.insert(t, v);
                        if i < pairings.len() {
                            let (t, v) = tlv::Type::Separator.as_type_value();
                            answer.insert(t, v);
                        }
                    }

                    debug!("/pairings - M2: Sending List Pairings Response");
                },
                _ => {},
            }
        }

        Box::new(future::ok(tlv_response(answer, StatusCode::Ok)))
    }
}
