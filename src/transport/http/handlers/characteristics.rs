use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use hyper::server::Response;
use hyper::{self, Uri};
use futures::{future, Future};

use accessory::HapAccessory;

use db::storage::Storage;
use db::database::Database;
use config::Config;
use transport::http::json_response;
use transport::http::handlers::Handler;
use transport::tlv;
use protocol::device::Device;
use protocol::pairing::Pairing;

struct Session {}

pub struct GetCharacteristics {
    session: Option<Session>
}

impl GetCharacteristics {
    pub fn new() -> GetCharacteristics {
        GetCharacteristics { session: None }
    }
}

impl<S: Storage> Handler<S> for GetCharacteristics {
    fn handle(&mut self, uri: Uri, body: Vec<u8>, database: &Arc<Mutex<Database<S>>>, accessories: &Arc<Vec<Box<HapAccessory>>>) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let decoded = tlv::decode(body);
        let answer = json!({
            "foo": "bar",
        });

        println!("/get-characteristics");

        Box::new(future::ok(json_response(answer)))
    }
}

pub struct UpdateCharacteristics {
    session: Option<Session>
}

impl UpdateCharacteristics {
    pub fn new() -> UpdateCharacteristics {
        UpdateCharacteristics { session: None }
    }
}

impl<S: Storage> Handler<S> for UpdateCharacteristics {
    fn handle(&mut self, uri: Uri, body: Vec<u8>, database: &Arc<Mutex<Database<S>>>, accessories: &Arc<Vec<Box<HapAccessory>>>) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let decoded = tlv::decode(body);
        let answer = json!({
            "foo": "bar",
        });

        println!("/update-characteristics");

        Box::new(future::ok(json_response(answer)))
    }
}
