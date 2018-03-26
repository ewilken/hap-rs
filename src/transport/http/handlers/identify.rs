use std::collections::HashMap;
use std::sync::{Arc, Mutex};
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
use transport::tlv;
use db::accessory_list::AccessoryList;
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
    fn handle(&mut self, uri: Uri, body: Vec<u8>, _: Arc<Option<Uuid>>, database: &Arc<Mutex<Database<S>>>, accessories: &AccessoryList) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let decoded = tlv::decode(body);
        let mut body: Vec<u8> = Vec::new();

        debug!("/identify");

        Box::new(future::ok(tlv_response(body, StatusCode::Ok)))
    }
}
