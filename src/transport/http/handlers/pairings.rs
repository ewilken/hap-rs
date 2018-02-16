use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use hyper::server::Response;
use hyper::{self, Uri};
use futures::{future, Future};

use accessory::Accessory;

use db::storage::Storage;
use db::database::Database;
use config::Config;
use transport::http::tlv_response;
use transport::http::handlers::Handler;
use transport::tlv;
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
    fn handle(&mut self, uri: Uri, body: Vec<u8>, database: &Arc<Mutex<Database<S>>>, accessories: &Arc<Vec<Accessory>>) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let decoded = tlv::decode(body);
        let mut answer: HashMap<u8, Vec<u8>> = HashMap::new();

        println!("/pairings");

        Box::new(future::ok(tlv_response(answer)))
    }
}
