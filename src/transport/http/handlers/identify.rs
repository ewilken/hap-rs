use std::sync::Arc;

use hyper::{self, Uri, StatusCode, server::Response};
use futures::{future, Future};
use serde_json;
use uuid::Uuid;

use config::Config;
use db::{accessory_list::AccessoryList, database::DatabasePtr};
use transport::http::{Status, json_response, status_response, handlers::Handler};

pub struct Identify {}

impl Identify {
    pub fn new() -> Identify {
        Identify {}
    }
}

impl Handler for Identify {
    fn handle(
        &mut self,
        _: Uri,
        _: Vec<u8>,
        _: Arc<Option<Uuid>>,
        database: &DatabasePtr,
        accessory_list: &AccessoryList,
    ) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let d = database.lock().unwrap();
        match d.list_pairings() {
            Ok(pairings) => {
                if pairings.len() > 0 {
                    let body = serde_json::to_vec(
                        &json!({"status": Status::InsufficientPrivileges as i32})
                    ).unwrap();
                    return Box::new(future::ok(json_response(body, StatusCode::BadRequest)));
                }

                let mut a = accessory_list.accessories.lock().unwrap();
                for accessory in a.iter_mut() {
                    accessory.get_mut_information().inner.identify.set_value(true).unwrap();
                }
            },
            Err(_) => {
                return Box::new(future::ok(status_response(StatusCode::InternalServerError)));
            },
        }

        Box::new(future::ok(status_response(StatusCode::NoContent)))
    }
}
