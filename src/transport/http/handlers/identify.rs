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
use protocol::device::Device;
use protocol::pairing::Pairing;

struct Session {}

pub struct Identify {
    session: Option<Session>
}

impl Identify {
    pub fn new() -> Identify {
        Identify { session: None }
    }
}

impl<S: Storage> Handler<S> for Identify {
    fn handle(&mut self, uri: Uri, body: Vec<u8>, database: &Arc<Mutex<Database<S>>>, accessories: &Arc<Vec<Box<HapAccessory>>>) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let decoded = tlv::decode(body);
        let mut answer: HashMap<u8, Vec<u8>> = HashMap::new();

        println!("/identify");

        Box::new(future::ok(tlv_response(answer)))
    }
}
