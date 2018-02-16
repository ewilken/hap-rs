use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use hyper::server::Response;
use hyper::{self, Uri};
use futures::{future, Future};
use serde_json;

use accessory::Accessory;

use db::storage::Storage;
use db::database::Database;
use config::Config;
use transport::http::json_response;
use transport::http::handlers::Handler;
use transport::tlv;
use protocol::device::Device;
use protocol::pairing::Pairing;

struct Session {}

pub struct Accessories {
    session: Option<Session>,
}

impl Accessories {
    pub fn new() -> Accessories {
        Accessories { session: None }
    }
}

impl<S: Storage> Handler<S> for Accessories {
    fn handle(&mut self, uri: Uri, body: Vec<u8>, database: &Arc<Mutex<Database<S>>>, accessories: &Arc<Vec<Accessory>>) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let accessories: Vec<serde_json::Value> = accessories.iter().map(|a| a.as_json()).collect();
        let answer = json!({
            "accessories": accessories,
        });

        Box::new(future::ok(json_response(answer)))
    }
}
