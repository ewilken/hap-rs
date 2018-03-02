use std::io::{self, Error};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::str;
use hyper::server::Response;
use hyper::{self, Uri, StatusCode};
use futures::{future, Future};
use uuid::Uuid;

use db::storage::Storage;
use db::database::Database;
use config::Config;
use transport::http::tlv_response;
use transport::http::handlers::Handler;
use transport::tlv::{self, Type, Value, ErrorKind};
use db::accessory_list::AccessoryList;
use protocol::pairing::{Pairing, Permissions};

pub struct Pairings {}

impl Pairings {
    pub fn new() -> Pairings {
        Pairings {}
    }
}

enum HandlerType {
    Add(Vec<u8>, Vec<u8>, Permissions),
    Remove(Vec<u8>),
    List,
}

impl HandlerType {
    fn parse(tlv_body: Vec<u8>) -> Result<HandlerType, Error> {
        let decoded = tlv::decode(tlv_body);
        if decoded.get(&(Type::State as u8)) != Some(&vec![1]) {
            return Err(Error::new(io::ErrorKind::Other, "invalid step"))
        }
        match decoded.get(&(Type::Method as u8)) {
            Some(handler) => match handler[0] {
                3 => {
                    let pairing_id = decoded.get(&(Type::Identifier as u8)).unwrap();
                    let ltpk = decoded.get(&(Type::PublicKey as u8)).unwrap();
                    let permissions = Permissions::from_u8(decoded.get(&(Type::Permissions as u8)).unwrap()[0]).unwrap();
                    Ok(HandlerType::Add(pairing_id.clone(), ltpk.clone(), permissions))
                },
                4 => {
                    let pairing_id = decoded.get(&(Type::Identifier as u8)).unwrap();
                    Ok(HandlerType::Remove(pairing_id.clone()))
                },
                5 => {
                    Ok(HandlerType::List)
                },
                _ => Err(Error::new(io::ErrorKind::Other, "invalid handler"))
            },
            None => Err(Error::new(io::ErrorKind::Other, "missing handler")),
        }
    }
}

impl<S: Storage> Handler<S> for Pairings {
    fn handle(&mut self, _: Uri, body: Vec<u8>, controller_id: Arc<Option<Uuid>>, database: &Arc<Mutex<Database<S>>>, _: &AccessoryList) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let mut answer: HashMap<u8, Vec<u8>> = HashMap::new();

        match HandlerType::parse(body) {
            Ok(HandlerType::Add(pairing_id, ltpk, permissions)) => {
                debug!("/pairings - M1: Got Add Pairing Request");

                let (t, v) = Value::State(2).as_tlv();
                answer.insert(t, v);

                // TODO - check if controller is admin

                let uuid_str = str::from_utf8(&pairing_id).unwrap();
                let pairing_uuid = Uuid::parse_str(uuid_str).unwrap();

                let d = database.lock().unwrap();
                match d.get_pairing(pairing_uuid) {
                    Ok(mut pairing) => {
                        if &pairing.public_key.to_vec() != &ltpk {
                            let (t, v) = Value::Error(ErrorKind::Unknown).as_tlv();
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
                            let (t, v) = Value::Error(ErrorKind::Unknown).as_tlv();
                            answer.insert(t, v);
                        }
                    },
                }

                debug!("/pairings - M2: Sending Add Pairing Response");
            },
            Ok(HandlerType::Remove(pairing_id)) => {
                debug!("/pairings - M1: Got Remove Pairing Request");

                let (t, v) = Value::State(2).as_tlv();
                answer.insert(t, v);

                let uuid_str = str::from_utf8(&pairing_id).unwrap();
                let pairing_uuid = Uuid::parse_str(uuid_str).unwrap();
                let d = database.lock().unwrap();
                match d.get_pairing(pairing_uuid) {
                    Ok(pairing) => {
                        if pairing.permissions != Permissions::Admin {
                            let (t, v) = Value::Error(ErrorKind::Authentication).as_tlv();
                            answer.insert(t, v);
                            return Box::new(future::ok(tlv_response(answer, StatusCode::Ok)));
                        }
                        if d.delete_pairing(&pairing.id).is_err() {
                            let (t, v) = Value::Error(ErrorKind::Unknown).as_tlv();
                            answer.insert(t, v);
                        }
                    },
                    Err(_) => {},
                }

                debug!("/pairings - M2: Sending Remove Pairing Response");
            },
            Ok(HandlerType::List) => {
                debug!("/pairings - M1: Got List Pairings Request");

                let (t, v) = Value::State(2).as_tlv();
                answer.insert(t, v);

                // TODO - check if controller is admin

                let d = database.lock().unwrap();
                let pairings = d.list_pairings().unwrap();
                for (i, pairing) in pairings.iter().enumerate() {
                    let (t, v) = Value::Identifier(pairing.id.hyphenated().to_string()).as_tlv();
                    answer.insert(t, v);
                    let (t, v) = Value::PublicKey(pairing.public_key.to_vec()).as_tlv();
                    answer.insert(t, v);
                    let (t, v) = Value::Permissions(pairing.permissions.clone()).as_tlv();
                    answer.insert(t, v);
                    if i < pairings.len() {
                        let (t, v) = Value::Separator.as_tlv();
                        answer.insert(t, v);
                    }
                }

                debug!("/pairings - M2: Sending List Pairings Response");
            },
            Err(_) => {
                // tlv error
            },
        }

        Box::new(future::ok(tlv_response(answer, StatusCode::Ok)))
    }
}
