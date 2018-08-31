use hyper::{Uri, StatusCode, server::Response};
use serde_json;

use config::ConfigPtr;
use db::{AccessoryList, DatabasePtr};
use transport::http::{
    Status,
    json_response,
    status_response,
    server::EventSubscriptions,
    handlers::JsonHandler,
};
use event::EmitterPtr;
use protocol::IdPtr;

use Error;

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
        _: &IdPtr,
        _: &EventSubscriptions,
        _: &ConfigPtr,
        database: &DatabasePtr,
        accessory_list: &AccessoryList,
        _: &EmitterPtr,
    ) -> Result<Response, Error> {
        if database.try_borrow()?.count_pairings()? > 0 {
            let body = serde_json::to_vec(
                &json!({"status": Status::InsufficientPrivileges as i32})
            )?;
            return Ok(json_response(body, StatusCode::BadRequest));
        }

        for accessory in accessory_list.accessories.try_borrow_mut()?.iter_mut() {
            accessory.try_borrow_mut()?.get_mut_information().inner.identify.set_value(true)?;
        }
        Ok(status_response(StatusCode::NoContent))
    }
}
