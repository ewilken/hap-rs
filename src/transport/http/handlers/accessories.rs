use std::sync::{Arc, Mutex};
use hyper::server::Response;
use hyper::{self, Uri, StatusCode};
use futures::{future, Future};
use serde_json;
use uuid::Uuid;

use db::database::DatabasePtr;
use transport::http::json_response;
use transport::http::handlers::Handler;
use db::accessory_list::AccessoryList;

pub struct Accessories {}

impl Accessories {
    pub fn new() -> Accessories {
        Accessories {}
    }
}

impl Handler for Accessories {
    fn handle(&mut self, _: Uri, _: Vec<u8>, _: Arc<Option<Uuid>>, _: &DatabasePtr, accessories: &AccessoryList) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let resp_body = serde_json::to_vec(accessories).unwrap();
        Box::new(future::ok(json_response(resp_body, StatusCode::Ok)))
    }
}
