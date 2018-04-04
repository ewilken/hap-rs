use std::sync::Arc;

use hyper::{self, Uri, StatusCode, server::Response};
use futures::{future, Future};
use serde_json;
use uuid::Uuid;

use db::{accessory_list::AccessoryList, database::DatabasePtr};
use transport::http::{handlers::Handler, json_response};

pub struct Accessories {}

impl Accessories {
    pub fn new() -> Accessories {
        Accessories {}
    }
}

impl Handler for Accessories {
    fn handle(
        &mut self,
        _: Uri,
        _: Vec<u8>,
        _: Arc<Option<Uuid>>,
        _: &DatabasePtr,
        accessories: &AccessoryList,
    ) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let resp_body = serde_json::to_vec(accessories).unwrap();
        Box::new(future::ok(json_response(resp_body, StatusCode::Ok)))
    }
}
