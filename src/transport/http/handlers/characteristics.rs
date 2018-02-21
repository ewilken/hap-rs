use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use hyper::server::Response;
use hyper::{self, Uri, StatusCode};
use futures::{future, Future};
use serde_json;

use accessory::HapAccessory;

use db::storage::Storage;
use db::database::Database;
use config::Config;
use transport::http::{json_response, status_response};
use transport::http::handlers::Handler;
use transport::http::Status;
use transport::tlv;
use db::accessory_list::AccessoryList;
use protocol::device::Device;
use protocol::pairing::Pairing;

struct Session {}

pub struct GetCharacteristics {
    session: Option<Session>
}

impl GetCharacteristics {
    pub fn new() -> GetCharacteristics {
        GetCharacteristics { session: None }
    }
}

impl<S: Storage> Handler<S> for GetCharacteristics {
    fn handle(&mut self, uri: Uri, _: Vec<u8>, database: &Arc<Mutex<Database<S>>>, accessories: &AccessoryList) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let resp_body = serde_json::to_vec(&json!({"foo": "bar"})).unwrap();
        println!("/get-characteristics");
        println!("uri: {:?}", uri);

        Box::new(future::ok(json_response(resp_body, StatusCode::Ok)))
    }
}

pub struct UpdateCharacteristics {
    session: Option<Session>
}

impl UpdateCharacteristics {
    pub fn new() -> UpdateCharacteristics {
        UpdateCharacteristics { session: None }
    }
}

impl<S: Storage> Handler<S> for UpdateCharacteristics {
    fn handle(&mut self, uri: Uri, body: Vec<u8>, database: &Arc<Mutex<Database<S>>>, accessories: &AccessoryList) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let write_body: Body<WriteObject> = serde_json::from_slice(&body).unwrap();
        let mut resp_body = Body::<WriteResponseObject> {
            characteristics: Vec::new()
        };
        let mut some_err = false;
        let mut all_err = true;

        for c in write_body.characteristics {
            let res_object = accessories.write_characteristic(c);
            if res_object.status != 0 {
                some_err = true;
            } else {
                all_err = false;
            }
            resp_body.characteristics.push(res_object);
        }

        if all_err {
            let res = serde_json::to_vec(&resp_body).unwrap();
            Box::new(future::ok(json_response(res, StatusCode::BadRequest)))
        } else if some_err {
            let res = serde_json::to_vec(&resp_body).unwrap();
            Box::new(future::ok(json_response(res, StatusCode::MultiStatus)))
        } else {
            Box::new(future::ok(status_response(StatusCode::NoContent)))
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Body<T> {
    characteristics: Vec<T>,
}

#[derive(Deserialize)]
pub struct WriteObject {
    pub iid: u64,
    pub aid: u64,
    pub value: Option<serde_json::Value>,
    pub ev: Option<bool>,
    #[serde(rename = "authData")]
    pub auth_data: Option<String>,
    pub remote: Option<bool>,
}

#[derive(Serialize)]
pub struct WriteResponseObject {
    pub iid: u64,
    pub aid: u64,
    pub status: i32,
}
