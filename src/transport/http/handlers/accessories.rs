use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use hyper::server::Response;
use hyper::{self, Uri};
use futures::{future, Future};
use serde_json;

use accessory::HapAccessory;

use db::storage::Storage;
use db::database::Database;
use transport::http::json_response;
use transport::http::handlers::Handler;

pub struct Accessories {}

impl Accessories {
    pub fn new() -> Accessories {
        Accessories {}
    }
}

impl<S: Storage> Handler<S> for Accessories {
    fn handle(&mut self, _: Uri, _: Vec<u8>, _: &Arc<Mutex<Database<S>>>, accessories: &Arc<Vec<Box<HapAccessory>>>) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let accessories: Vec<serde_json::Value> = accessories.iter().map(|a| a.to_json()).collect();
        let answer = json!({
            "accessories": accessories,
        });

        Box::new(future::ok(json_response(answer)))
    }
}
