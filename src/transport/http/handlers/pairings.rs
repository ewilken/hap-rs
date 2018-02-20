use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use hyper::server::Response;
use hyper::{self, Uri};
use futures::{future, Future};

use accessory::HapAccessory;

use db::storage::Storage;
use db::database::Database;
use config::Config;
use transport::http::tlv_response;
use transport::http::handlers::Handler;
use transport::tlv;
use transport::accessory_list::AccessoryList;
use protocol::device::Device;
use protocol::pairing::Pairing;

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
    fn handle(&mut self, uri: Uri, body: Vec<u8>, database: &Arc<Mutex<Database<S>>>, accessories: &AccessoryList) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let decoded = tlv::decode(body);
        let mut answer: HashMap<u8, Vec<u8>> = HashMap::new();

        if let (Some(v), Some(m)) = (decoded.get(&0x06), decoded.get(&0x00)) {
            match (v[0], m[0]) {
                (1, 3) => {
                    println!("/pairings - M1: Got Add Pairing Request");
                },
                (1, 4) => {
                    println!("/pairings - M1: Got Remove Pairing Request");

                    let (t, v) = tlv::Type::State(2).as_type_value();
                    answer.insert(t, v);

                    let pairing_id = decoded.get(&0x01).unwrap();
                },
                (1, 5) => {
                    println!("/pairings - M1: Got List Pairings Request");
                },
                _ => {},
            }
        }

        Box::new(future::ok(tlv_response(answer)))
    }
}
