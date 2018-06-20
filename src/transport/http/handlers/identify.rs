use std::sync::Arc;

use hyper::{Uri, StatusCode, server::Response};
use failure::Error;
use serde_json;
use uuid::Uuid;

use config::ConfigPtr;
use db::{accessory_list::AccessoryList, database::DatabasePtr};
use transport::http::{
    Status,
    json_response,
    status_response,
    server::EventSubscriptions,
    handlers::JsonHandler,
};
use event::EmitterPtr;

pub struct Identify {}

impl Identify {
    pub fn new() -> Identify {
        Identify {}
    }
}

impl JsonHandler for Identify {
    fn handle(
        &mut self,
        _: Uri,
        _: Vec<u8>,
        _: Arc<Option<Uuid>>,
        _: &EventSubscriptions,
        _: &ConfigPtr,
        database: &DatabasePtr,
        accessory_list: &AccessoryList,
        _: &EmitterPtr,
    ) -> Result<Response, Error> {
        let d = database.lock().unwrap();
        let count = d.count_pairings()?;
        if count > 0 {
            let body = serde_json::to_vec(
                &json!({"status": Status::InsufficientPrivileges as i32})
            ).unwrap();
            return Ok(json_response(body, StatusCode::BadRequest));
        }

        let mut a = accessory_list.accessories.lock().unwrap();
        for accessory in a.iter_mut() {
            accessory.get_mut_information().inner.identify.set_value(true)?;
        }
        Ok(status_response(StatusCode::NoContent))
    }
}
