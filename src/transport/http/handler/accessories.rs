use hyper::{Body, Response, StatusCode, Uri};

use crate::{
    config::ConfigPtr,
    db::{AccessoryList, DatabasePtr},
    event::EventEmitterPtr,
    protocol::IdPtr,
    transport::http::{handler::JsonHandler, json_response, server::EventSubscriptions},
    Result,
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
        _: &EventEmitterPtr,
    ) -> Result<Response<Body>> {
        let resp_body = serde_json::to_vec(accessories)?;
        json_response(resp_body, StatusCode::OK)
    }
}
