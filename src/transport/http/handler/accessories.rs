use futures::future::{BoxFuture, FutureExt};
use hyper::{Body, Response, StatusCode, Uri};

use crate::{
    pointer,
    transport::http::{handler::JsonHandlerExt, json_response},
    Result,
};

pub struct Accessories;

impl Accessories {
    pub fn new() -> Accessories { Accessories }
}

impl JsonHandlerExt for Accessories {
    fn handle(
        &mut self,
        _: Uri,
        _: Body,
        _: pointer::ControllerId,
        _: pointer::EventSubscriptions,
        _: pointer::Config,
        _: pointer::Storage,
        accessory_list: pointer::AccessoryList,
        _: pointer::EventEmitter,
    ) -> BoxFuture<Result<Response<Body>>> {
        async move {
            let resp_body = accessory_list.lock().await.as_serialized_json().await?;
            // let resp_body = serde_json::to_vec(&accessory_list)?;
            json_response(resp_body, StatusCode::OK)
        }
        .boxed()
    }
}
