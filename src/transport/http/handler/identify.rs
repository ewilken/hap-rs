use futures::future::{BoxFuture, FutureExt};
use hyper::{Body, Response, StatusCode, Uri};
use serde_json::json;

use crate::{
    pointer,
    transport::http::{handler::JsonHandlerExt, json_response, status_response, Status},
    Result,
};

pub struct Identify;

impl Identify {
    pub fn new() -> Identify { Identify }
}

impl JsonHandlerExt for Identify {
    fn handle(
        &mut self,
        _: Uri,
        _: Body,
        _: pointer::ControllerId,
        _: pointer::EventSubscriptions,
        _: pointer::Config,
        storage: pointer::Storage,
        accessory_list: pointer::AccessoryList,
        _: pointer::EventEmitter,
    ) -> BoxFuture<Result<Response<Body>>> {
        let storage = storage.clone();
        let accessory_list = accessory_list.clone();

        async move {
            if storage.lock().expect("couldn't access storage").count_pairings()? > 0 {
                let body = serde_json::to_vec(&json!({ "status": Status::InsufficientPrivileges as i32 }))?;
                return json_response(body, StatusCode::BAD_REQUEST);
            }

            for accessory in accessory_list
                .lock()
                .expect("couldn't access accessory_list")
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
        .boxed()
    }
}
