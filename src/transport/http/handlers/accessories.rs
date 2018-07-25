use std::rc::Rc;

use hyper::{Uri, StatusCode, server::Response};
use failure::Error;
use serde_json;
use uuid::Uuid;

use config::ConfigPtr;
use db::{AccessoryList, DatabasePtr};
use transport::http::{server::EventSubscriptions, handlers::JsonHandler, json_response};
use event::EmitterPtr;

pub struct Accessories {}

impl Accessories {
    pub fn new() -> Accessories {
        Accessories {}
    }
}

impl JsonHandler for Accessories {
    fn handle(
        &mut self,
        _: Uri,
        _: Vec<u8>,
        _: Rc<Option<Uuid>>,
        _: &EventSubscriptions,
        _: &ConfigPtr,
        _: &DatabasePtr,
        accessories: &AccessoryList,
        _: &EmitterPtr,
    ) -> Result<Response, Error> {
        let resp_body = serde_json::to_vec(accessories)?;
        Ok(json_response(resp_body, StatusCode::Ok))
    }
}
