use futures::future::{BoxFuture, FutureExt};
use hyper::{self, Body, Response, StatusCode, Uri};

use crate::{
    pointer,
    tlv::{self, Encodable},
    transport::http::{status_response, tlv_response},
    ErrorKind,
    Result,
};

pub mod accessories;
pub mod characteristics;
pub mod identify;
pub mod pair_setup;
pub mod pair_verify;
pub mod pairings;

pub trait HandlerExt {
    fn handle(
        &mut self,
        uri: Uri,
        body: Body,
        controller_id: pointer::ControllerId,
        event_subscriptions: pointer::EventSubscriptions,
        config: pointer::Config,
        storage: pointer::Storage,
        accessory_list: pointer::AccessoryList,
        event_emitter: pointer::EventEmitter,
    ) -> BoxFuture<Result<Response<Body>>>;
}

pub trait TlvHandlerExt {
    type ParseResult: Send;
    type Result: Encodable;

    fn parse(&self, body: Body) -> BoxFuture<std::result::Result<Self::ParseResult, tlv::ErrorContainer>>;
    fn handle(
        &mut self,
        step: Self::ParseResult,
        controller_id: pointer::ControllerId,
        config: pointer::Config,
        storage: pointer::Storage,
        event_emitter: pointer::EventEmitter,
    ) -> BoxFuture<std::result::Result<Self::Result, tlv::ErrorContainer>>;
}

#[derive(Debug)]
pub struct TlvHandler<T: TlvHandlerExt + Send + Sync>(T);

impl<T: TlvHandlerExt + Send + Sync> From<T> for TlvHandler<T> {
    fn from(inst: T) -> TlvHandler<T> { TlvHandler(inst) }
}

impl<T: TlvHandlerExt + Send + Sync> HandlerExt for TlvHandler<T> {
    fn handle(
        &mut self,
        _: Uri,
        body: Body,
        controller_id: pointer::ControllerId,
        _: pointer::EventSubscriptions,
        config: pointer::Config,
        storage: pointer::Storage,
        _: pointer::AccessoryList,
        event_emitter: pointer::EventEmitter,
    ) -> BoxFuture<Result<Response<Body>>> {
        async move {
            let response = match self.0.parse(body).await {
                Err(e) => e.encode(),
                Ok(step) => match self.0.handle(step, controller_id, config, storage, event_emitter).await {
                    Err(e) => e.encode(),
                    Ok(res) => res.encode(),
                },
            };
            tlv_response(response, StatusCode::OK)
        }
        .boxed()
    }
}

pub trait JsonHandlerExt {
    fn handle(
        &mut self,
        uri: Uri,
        body: Body,
        controller_id: pointer::ControllerId,
        event_subscriptions: pointer::EventSubscriptions,
        config: pointer::Config,
        storage: pointer::Storage,
        accessory_list: pointer::AccessoryList,
        event_emitter: pointer::EventEmitter,
    ) -> BoxFuture<Result<Response<Body>>>;
}

#[derive(Debug)]
pub struct JsonHandler<T: JsonHandlerExt + Send + Sync>(T);

impl<T: JsonHandlerExt + Send + Sync> From<T> for JsonHandler<T> {
    fn from(inst: T) -> JsonHandler<T> { JsonHandler(inst) }
}

impl<T: JsonHandlerExt + Send + Sync> HandlerExt for JsonHandler<T> {
    fn handle(
        &mut self,
        uri: Uri,
        body: Body,
        controller_id: pointer::ControllerId,
        event_subscriptions: pointer::EventSubscriptions,
        config: pointer::Config,
        storage: pointer::Storage,
        accessory_list: pointer::AccessoryList,
        event_emitter: pointer::EventEmitter,
    ) -> BoxFuture<Result<Response<Body>>> {
        async move {
            match self
                .0
                .handle(
                    uri,
                    body,
                    controller_id,
                    event_subscriptions,
                    config,
                    storage,
                    accessory_list,
                    event_emitter,
                )
                .await
            {
                Ok(res) => Ok(res),
                Err(e) => match e.kind() {
                    &ErrorKind::HttpStatus(status) => status_response(status),
                    _ => status_response(StatusCode::INTERNAL_SERVER_ERROR),
                },
            }
        }
        .boxed()
    }
}
