use hyper::{self, Uri, StatusCode, server::Response};
use futures::{future, Future};

use config::ConfigPtr;
use db::{DatabasePtr, AccessoryList};
use transport::http::{tlv_response, status_response, server::EventSubscriptions};
use protocol::{tlv::{self, Encodable}, IdPtr};
use event::EmitterPtr;

use Error;

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
        event_emitter: &EmitterPtr,
    ) -> Box<Future<Item=Response, Error=hyper::Error>>;
}

pub trait TlvHandler {
    type ParseResult;
    type Result: Encodable;
    fn parse(&self, body: Vec<u8>) -> Result<Self::ParseResult, tlv::ErrorContainer>;
    fn handle(
        &mut self,
        step: Self::ParseResult,
        controller_id: &IdPtr,
        config: &ConfigPtr,
        database: &DatabasePtr,
        event_emitter: &EmitterPtr,
    ) -> Result<Self::Result, tlv::ErrorContainer>;
}

pub struct TlvHandlerType<T: TlvHandler>(T);

impl<T: TlvHandler> From<T> for TlvHandlerType<T> {
    fn from(inst: T) -> TlvHandlerType<T> {
        TlvHandlerType(inst)
    }
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
        event_emitter: &EmitterPtr,
    ) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let response = match self.0.parse(body) {
            Err(e) => e.encode(),
            Ok(step) => match self.0.handle(step, controller_id, config, database, event_emitter) {
                Err(e) => e.encode(),
                Ok(res) => res.encode(),
            }
        };
        Box::new(future::ok(tlv_response(response, StatusCode::Ok)))
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
        event_emitter: &EmitterPtr,
    ) -> Result<Response, Error>;
}

pub struct JsonHandlerType<T: JsonHandler>(T);

impl<T: JsonHandler> From<T> for JsonHandlerType<T> {
    fn from(inst: T) -> JsonHandlerType<T> {
        JsonHandlerType(inst)
    }
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
        event_emitter: &EmitterPtr,
    ) -> Box<Future<Item=Response, Error=hyper::Error>> {
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
            Ok(res) => res,
            Err(e) => match e {
                Error::HttpStatus(status) => status_response(status),
                _ => status_response(StatusCode::InternalServerError),
            },
        };
        Box::new(future::ok(response))
    }
}
