use futures::future::{BoxFuture, FutureExt};
use hyper::{body::Buf, Body, Response, StatusCode, Uri};
use std::collections::HashMap;
use url::form_urlencoded;

use crate::{
    pointer,
    transport::http::{
        handler::JsonHandlerExt,
        json_response,
        status_response,
        CharacteristicResponseBody,
        ReadResponseObject,
        Status,
        WriteObject,
        WriteResponseObject,
    },
    Error,
    Result,
};

pub struct GetCharacteristics;

impl GetCharacteristics {
    pub fn new() -> Self { GetCharacteristics }
}

impl JsonHandlerExt for GetCharacteristics {
    fn handle(
        &mut self,
        uri: Uri,
        _: Body,
        _: pointer::ControllerId,
        _: pointer::EventSubscriptions,
        _: pointer::Config,
        _: pointer::Storage,
        accessory_list: pointer::AccessoryList,
        _: pointer::EventEmitter,
    ) -> BoxFuture<Result<Response<Body>>> {
        async move {
            if let Some(query) = uri.query() {
                let mut resp_body = CharacteristicResponseBody::<ReadResponseObject> {
                    characteristics: Vec::new(),
                };
                let mut some_err = false;

                let mut queries: HashMap<String, String> = HashMap::new();
                for (key, val) in form_urlencoded::parse(query.as_bytes()) {
                    queries.insert(key.into(), val.into());
                }
                let (f_meta, f_perms, f_type, f_ev) = check_flags(&queries);
                let q_id = queries.get("id").ok_or(Error::HttpStatus(StatusCode::BAD_REQUEST))?;
                let ids = q_id.split(',').collect::<Vec<&str>>();
                for id in ids {
                    let id_pair = id.split('.').collect::<Vec<&str>>();
                    if id_pair.len() != 2 {
                        return Err(Error::HttpStatus(StatusCode::BAD_REQUEST));
                    }
                    let aid = id_pair[0].parse::<u64>()?;
                    let iid = id_pair[1].parse::<u64>()?;

                    let res_object = match accessory_list
                        .lock()
                        .await
                        .read_characteristic(aid, iid, f_meta, f_perms, f_type, f_ev)
                        .await
                    {
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
                    return json_response(res, StatusCode::MULTI_STATUS);
                }
                for ref mut r in &mut resp_body.characteristics {
                    r.status = None;
                }
                let res = serde_json::to_vec(&resp_body)?;

                json_response(res, StatusCode::OK)
            } else {
                status_response(StatusCode::BAD_REQUEST)
            }
        }
        .boxed()
    }
}

fn check_flags(flags: &HashMap<String, String>) -> (bool, bool, bool, bool) {
    let true_val = "1".to_string();
    (
        flags.get("meta") == Some(&true_val),
        flags.get("perms") == Some(&true_val),
        flags.get("type") == Some(&true_val),
        flags.get("ev") == Some(&true_val),
    )
}

pub struct UpdateCharacteristics;

impl UpdateCharacteristics {
    pub fn new() -> Self { UpdateCharacteristics {} }
}

impl JsonHandlerExt for UpdateCharacteristics {
    fn handle(
        &mut self,
        _: Uri,
        body: Body,
        _: pointer::ControllerId,
        event_subscriptions: pointer::EventSubscriptions,
        _: pointer::Config,
        _: pointer::Storage,
        accessories: pointer::AccessoryList,
        _: pointer::EventEmitter,
    ) -> BoxFuture<Result<Response<Body>>> {
        async move {
            let aggregated_body = hyper::body::aggregate(body).await?;

            let write_body: CharacteristicResponseBody<WriteObject> = serde_json::from_slice(aggregated_body.chunk())?;
            let mut resp_body = CharacteristicResponseBody::<WriteResponseObject> {
                characteristics: Vec::new(),
            };
            let mut some_err = false;
            let mut all_err = true;

            for c in write_body.characteristics {
                let iid = c.iid;
                let aid = c.aid;
                let res_object = match accessories
                    .lock()
                    .await
                    .write_characteristic(c, &event_subscriptions)
                    .await
                {
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
                            iid,
                            aid,
                            status: Status::ServiceCommunicationFailure as i32,
                        }
                    },
                };

                resp_body.characteristics.push(res_object);
            }

            if all_err {
                let res = serde_json::to_vec(&resp_body)?;
                json_response(res, StatusCode::BAD_REQUEST)
            } else if some_err {
                let res = serde_json::to_vec(&resp_body)?;
                json_response(res, StatusCode::MULTI_STATUS)
            } else {
                status_response(StatusCode::NO_CONTENT)
            }
        }
        .boxed()
    }
}
