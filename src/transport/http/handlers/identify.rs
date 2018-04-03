use std::sync::Arc;

use hyper::{self, Uri, StatusCode, server::Response};
use futures::{future, Future};
use uuid::Uuid;

use accessory::HapAccessory;
use db::storage::Storage;
use db::database::DatabasePtr;
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

impl Handler for Identify {
    fn handle(&mut self, uri: Uri, body: Vec<u8>, _: Arc<Option<Uuid>>, database: &DatabasePtr, accessories: &AccessoryList) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let decoded = tlv::decode(body);
        let mut body: Vec<u8> = Vec::new();

        println!("/identify");

        Box::new(future::ok(tlv_response(body, StatusCode::Ok)))
    }
}
