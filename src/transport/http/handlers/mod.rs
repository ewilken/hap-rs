use std::sync::Arc;

use hyper::{self, Uri, Error, StatusCode, server::Response};
use futures::{future, Future};
use uuid::Uuid;

use db::{database::DatabasePtr, accessory_list::AccessoryList};
use transport::{http::tlv_response, tlv::{self, Encodable}};

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
        controller_id: Arc<Option<Uuid>>,
        database: &DatabasePtr,
        accessories: &AccessoryList,
    ) -> Box<Future<Item=Response, Error=Error>>;
}

pub trait TlvHandler {
    type ParseResult;
    type Result: Encodable;
    fn parse(&self, body: Vec<u8>) -> Result<Self::ParseResult, tlv::ErrorContainer>;
    fn handle(&mut self, step: Self::ParseResult, database: &DatabasePtr) -> Result<Self::Result, tlv::ErrorContainer>;
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
        _: Arc<Option<Uuid>>,
        database: &DatabasePtr,
        _: &AccessoryList,
    ) -> Box<Future<Item=Response, Error=hyper::Error>> {
        let response = match self.0.parse(body) {
            Err(e) => e.encode(),
            Ok(step) => match self.0.handle(step, database) {
                Err(e) => e.encode(),
                Ok(res) => res.encode(),
            }
        };
        Box::new(future::ok(tlv_response(response, StatusCode::Ok)))
    }
}
