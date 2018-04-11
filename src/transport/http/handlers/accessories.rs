use std::sync::Arc;

use hyper::{Uri, StatusCode, server::Response};
use failure::Error;
use serde_json;
use uuid::Uuid;

use db::{accessory_list::AccessoryList, database::DatabasePtr};
use transport::http::{handlers::JsonHandler, json_response};

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
        _: Arc<Option<Uuid>>,
        _: &DatabasePtr,
        accessories: &AccessoryList,
    ) -> Result<Response, Error> {
        let resp_body = serde_json::to_vec(accessories)?;
        Ok(json_response(resp_body, StatusCode::Ok))
    }
}
