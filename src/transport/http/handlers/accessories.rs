use hyper::{Uri, StatusCode, server::Response};
use serde_json;

use config::ConfigPtr;
use db::{AccessoryList, DatabasePtr};
use transport::http::{server::EventSubscriptions, handlers::JsonHandler, json_response};
use event::EmitterPtr;
use protocol::IdPtr;

use Error;

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
        _: &IdPtr,
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
