use hyper::{Body, Response, StatusCode, Uri};
use serde_json;

use crate::{
    config::ConfigPtr,
    db::{AccessoryList, DatabasePtr},
    event::EmitterPtr,
    protocol::IdPtr,
    transport::http::{handler::JsonHandler, json_response, server::EventSubscriptions},
    Error,
};

pub struct Accessories;

impl Accessories {
    pub fn new() -> Accessories { Accessories }
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
    ) -> Result<Response<Body>, Error> {
        let resp_body = serde_json::to_vec(accessories)?;
        json_response(resp_body, StatusCode::OK)
    }
}
