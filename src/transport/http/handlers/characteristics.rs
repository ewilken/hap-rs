use std::collections::HashMap;

use hyper::{Uri, StatusCode, server::Response};
use serde_json;
use url::form_urlencoded;

use db::{AccessoryList, DatabasePtr};
use config::ConfigPtr;
use transport::http::{
    Status,
    CharacteristicResponseBody,
    ReadResponseObject,
    WriteObject,
    WriteResponseObject,
    server::EventSubscriptions,
    handlers::JsonHandler,
    json_response,
    status_response,
};
use event::EmitterPtr;
use protocol::IdPtr;

use Error;

pub struct GetCharacteristics {}

impl GetCharacteristics {
    pub fn new() -> GetCharacteristics {
        GetCharacteristics {}
    }
}

impl JsonHandler for GetCharacteristics {
    fn handle(
        &mut self,
        uri: Uri,
        _: Vec<u8>,
        _: &IdPtr,
        _: &EventSubscriptions,
        _: &ConfigPtr,
        _: &DatabasePtr,
        accessories: &AccessoryList,
        _: &EmitterPtr,
    ) -> Result<Response, Error> {
        if let Some(query) = uri.query() {
            let mut resp_body = CharacteristicResponseBody::<ReadResponseObject> {
                characteristics: Vec::new()
            };
            let mut some_err = false;

            // TODO - using a String seems ugly
            let mut queries: HashMap<String, String> = HashMap::new();
            for (key, val) in form_urlencoded::parse(query.as_bytes()) {
                queries.insert(key.into(), val.into());
            }
            let (f_meta, f_perms, f_type, f_ev) = check_flags(&queries);
            let q_id = queries.get("id").ok_or(Error::HttpStatus(StatusCode::BadRequest))?;
            let ids = q_id.split(",").collect::<Vec<&str>>();
            for id in ids {
                let id_pair = id.split(".").collect::<Vec<&str>>();
                if id_pair.len() != 2 {
                    return Err(Error::HttpStatus(StatusCode::BadRequest));
                }
                let aid = id_pair[0].parse::<u64>()?;
                let iid = id_pair[1].parse::<u64>()?;

                let res_object = match accessories.read_characteristic(
                    aid,
                    iid,
                    f_meta,
                    f_perms,
                    f_type,
                    f_ev,
                ) {
                    Ok(mut res_object) => {
                        if res_object.status != Some(0) {
                            some_err = true;
                            res_object.value = None;
                        }
                        res_object
                    },
                    Err(_) => {
                        some_err = true;
                        ReadResponseObject {
                            iid,
                            aid,
                            status: Some(Status::ServiceCommunicationFailure as i32),
                            ..Default::default()
                        }
                    },
                };

                resp_body.characteristics.push(res_object);
            }

            if some_err {
                let res = serde_json::to_vec(&resp_body)?;
                return Ok(json_response(res, StatusCode::MultiStatus));
            }
            for ref mut r in &mut resp_body.characteristics {
                r.status = None;
            }
            let res = serde_json::to_vec(&resp_body)?;
            Ok(json_response(res, StatusCode::Ok))
        } else {
            Ok(status_response(StatusCode::BadRequest))
        }
    }
}

fn check_flags(flags: &HashMap<String, String>) -> (bool, bool, bool, bool) {
    let true_val = "1".to_owned();
    (
        flags.get("meta") == Some(&true_val),
        flags.get("perms") == Some(&true_val),
        flags.get("type") == Some(&true_val),
        flags.get("ev") == Some(&true_val),
    )
}

pub struct UpdateCharacteristics {}

impl UpdateCharacteristics {
    pub fn new() -> UpdateCharacteristics {
        UpdateCharacteristics {}
    }
}

impl JsonHandler for UpdateCharacteristics {
    fn handle(
        &mut self,
        _: Uri,
        body: Vec<u8>,
        _: &IdPtr,
        event_subscriptions: &EventSubscriptions,
        _: &ConfigPtr,
        _: &DatabasePtr,
        accessories: &AccessoryList,
        _: &EmitterPtr,
    ) -> Result<Response, Error> {
        let write_body: CharacteristicResponseBody<WriteObject> = serde_json::from_slice(&body)?;
        let mut resp_body = CharacteristicResponseBody::<WriteResponseObject> {
            characteristics: Vec::new()
        };
        let mut some_err = false;
        let mut all_err = true;

        for c in write_body.characteristics {
            let iid = c.iid;
            let aid = c.aid;
            let res_object = match accessories.write_characteristic(c, event_subscriptions) {
                Ok(res_object) => {
                    if res_object.status != 0 {
                        some_err = true;
                    } else {
                        all_err = false;
                    }
                    res_object
                },
                Err(_) => {
                    some_err = true;
                    WriteResponseObject {
                        iid: iid,
                        aid: aid,
                        status: Status::ServiceCommunicationFailure as i32,
                    }
                },
            };

            resp_body.characteristics.push(res_object);
        }

        if all_err {
            let res = serde_json::to_vec(&resp_body)?;
            Ok(json_response(res, StatusCode::BadRequest))
        } else if some_err {
            let res = serde_json::to_vec(&resp_body)?;
            Ok(json_response(res, StatusCode::MultiStatus))
        } else {
            Ok(status_response(StatusCode::NoContent))
        }
    }
}
