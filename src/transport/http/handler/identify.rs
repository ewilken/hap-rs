use hyper::{Body, Response, StatusCode, Uri};
use serde_json::{self, json};

use crate::{
    config::ConfigPtr,
    db::{AccessoryList, DatabasePtr},
    event::EmitterPtr,
    protocol::IdPtr,
    transport::http::{handler::JsonHandler, json_response, server::EventSubscriptions, status_response, Status},
    Error,
};

pub struct Identify;

impl Identify {
    pub fn new() -> Identify { Identify }
}

impl JsonHandler for Identify {
    fn handle(
        &mut self,
        _: Uri,
        _: Vec<u8>,
        _: &IdPtr,
        _: &EventSubscriptions,
        _: &ConfigPtr,
        database: &DatabasePtr,
        accessory_list: &AccessoryList,
        _: &EmitterPtr,
    ) -> Result<Response<Body>, Error> {
        if database.lock().expect("couldn't access database").count_pairings()? > 0 {
            let body = serde_json::to_vec(&json!({ "status": Status::InsufficientPrivileges as i32 }))?;
            return json_response(body, StatusCode::BAD_REQUEST);
        }

        for accessory in accessory_list
            .accessories
            .lock()
            .expect("couldn't access accessory_list")
            .iter_mut()
        {
            accessory
                .lock()
                .expect("couldn't access accessory")
                .get_mut_information()
                .inner
                .identify
                .set_value(true)?;
        }

        status_response(StatusCode::NO_CONTENT)
    }
}
