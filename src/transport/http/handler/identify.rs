use futures::future::{BoxFuture, FutureExt};
use hyper::{Body, Response, StatusCode, Uri};
use serde_json::json;

use crate::{
    pointer,
    transport::http::{handler::JsonHandlerExt, json_response, status_response, Status},
    HapType,
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
        accessory_database: pointer::AccessoryDatabase,
        _: pointer::EventEmitter,
    ) -> BoxFuture<Result<Response<Body>>> {
        let storage = storage.clone();
        let accessory_database = accessory_database;

        async move {
            if storage.lock().await.count_pairings().await? > 0 {
                let body = serde_json::to_vec(&json!({ "status": Status::InsufficientPrivileges as i32 }))?;
                return json_response(body, StatusCode::BAD_REQUEST);
            }

            for accessory in accessory_database.lock().await.accessories.iter_mut() {
                accessory
                    .lock()
                    .await
                    .get_mut_service(HapType::AccessoryInformation)
                    .expect("missing Accessory Information Service") // every accessory needs to have it, so this should never panic
                    .get_mut_characteristic(HapType::Identify)
                    .expect("missing Identify Characteristic on Accessory Information Service")
                    .set_value(serde_json::Value::Bool(true))
                    .await?;
            }

            // TODO: defer setting them all back to false after a few secs

            status_response(StatusCode::NO_CONTENT)
        }
        .boxed()
    }
}
