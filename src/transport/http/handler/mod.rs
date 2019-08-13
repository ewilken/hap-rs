use futures::{future, Future};
use hyper::{self, Body, Response, StatusCode, Uri};

use crate::{
    config::ConfigPtr,
    db::{AccessoryList, DatabasePtr},
    event::EventEmitterPtr,
    protocol::{
        tlv::{self, Encodable},
        IdPtr,
    },
    transport::http::{server::EventSubscriptions, status_response, tlv_response},
    Error,
    ErrorKind,
    Result,
};

pub mod accessories;
pub mod characteristics;
pub mod identify;
pub mod pair_setup;
pub mod pair_verify;
pub mod pairings;

pub trait Handler {
    fn handle(
        &mut self,
        uri: Uri,
        body: Vec<u8>,
        controller_id: &IdPtr,
        event_subscriptions: &EventSubscriptions,
        config: &ConfigPtr,
        database: &DatabasePtr,
        accessories: &AccessoryList,
        event_emitter: &EventEmitterPtr,
    ) -> Box<dyn Future<Item = Response<Body>, Error = Error> + Send>;
}

pub trait TlvHandler {
    type ParseResult;
    type Result: Encodable;
    fn parse(&self, body: Vec<u8>) -> std::result::Result<Self::ParseResult, tlv::ErrorContainer>;
    fn handle(
        &mut self,
        step: Self::ParseResult,
        controller_id: &IdPtr,
        config: &ConfigPtr,
        database: &DatabasePtr,
        event_emitter: &EventEmitterPtr,
    ) -> std::result::Result<Self::Result, tlv::ErrorContainer>;
}

pub struct TlvHandlerType<T: TlvHandler>(T);

impl<T: TlvHandler> From<T> for TlvHandlerType<T> {
    fn from(inst: T) -> TlvHandlerType<T> { TlvHandlerType(inst) }
}

impl<T: TlvHandler> Handler for TlvHandlerType<T> {
    fn handle(
        &mut self,
        _: Uri,
        body: Vec<u8>,
        controller_id: &IdPtr,
        _: &EventSubscriptions,
        config: &ConfigPtr,
        database: &DatabasePtr,
        _: &AccessoryList,
        event_emitter: &EventEmitterPtr,
    ) -> Box<dyn Future<Item = Response<Body>, Error = Error> + Send> {
        let response = match self.0.parse(body) {
            Err(e) => e.encode(),
            Ok(step) => match self.0.handle(step, controller_id, config, database, event_emitter) {
                Err(e) => e.encode(),
                Ok(res) => res.encode(),
            },
        };
        Box::new(future::result(
            tlv_response(response, StatusCode::OK).map_err(Error::from),
        ))
    }
}

pub trait JsonHandler {
    fn handle(
        &mut self,
        uri: Uri,
        body: Vec<u8>,
        controller_id: &IdPtr,
        event_subscriptions: &EventSubscriptions,
        config: &ConfigPtr,
        database: &DatabasePtr,
        accessory_list: &AccessoryList,
        event_emitter: &EventEmitterPtr,
    ) -> Result<Response<Body>>;
}

pub struct JsonHandlerType<T: JsonHandler>(T);

impl<T: JsonHandler> From<T> for JsonHandlerType<T> {
    fn from(inst: T) -> JsonHandlerType<T> { JsonHandlerType(inst) }
}

impl<T: JsonHandler> Handler for JsonHandlerType<T> {
    fn handle(
        &mut self,
        uri: Uri,
        body: Vec<u8>,
        controller_id: &IdPtr,
        event_subscriptions: &EventSubscriptions,
        config: &ConfigPtr,
        database: &DatabasePtr,
        accessory_list: &AccessoryList,
        event_emitter: &EventEmitterPtr,
    ) -> Box<dyn Future<Item = Response<Body>, Error = Error> + Send> {
        let response = match self.0.handle(
            uri,
            body,
            controller_id,
            event_subscriptions,
            config,
            database,
            accessory_list,
            event_emitter,
        ) {
            Ok(res) => Ok(res),
            Err(e) => match e.kind() {
                &ErrorKind::HttpStatus(status) => status_response(status),
                _ => status_response(StatusCode::INTERNAL_SERVER_ERROR),
            },
        };
        Box::new(future::result(response))
    }
}
