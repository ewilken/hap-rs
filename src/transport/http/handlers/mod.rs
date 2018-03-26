use std::sync::{Arc, Mutex};
use std::marker::PhantomData;

use hyper::{self, Uri, Error, StatusCode};
use hyper::server::Response;
use futures::{future, Future};
use uuid::Uuid;

use db::storage::Storage;
use db::database::Database;
use db::accessory_list::AccessoryList;
use transport::http::tlv_response;
use transport::tlv::{self, Encodable};

pub mod accessories;
pub mod characteristics;
pub mod identify;
pub mod pair_setup;
pub mod pair_verify;
pub mod pairings;

pub trait Handler<S: Storage> {
    fn handle(
        &mut self,
        uri: Uri,
        body: Vec<u8>,
        controller_id: Arc<Option<Uuid>>,
        database: &Arc<Mutex<Database<S>>>,
        accessories: &AccessoryList,
    ) -> Box<Future<Item=Response, Error=Error>>;
}

pub trait TlvHandler<S: Storage> {
    type ParseResult;
    type Result: Encodable;
    fn parse(&self, body: Vec<u8>) -> Result<Self::ParseResult, tlv::ErrorContainer>;
    fn handle(&mut self, step: Self::ParseResult, database: &Arc<Mutex<Database<S>>>) -> Result<Self::Result, tlv::ErrorContainer>;
}

pub struct TlvHandlerType<T: TlvHandler<S>, S:Storage>(T, PhantomData<S>);

impl<T: TlvHandler<S>, S: Storage> From<T> for TlvHandlerType<T, S> {
    fn from(inst: T) -> TlvHandlerType<T, S> {
        TlvHandlerType(inst, PhantomData::default())
    }
}

impl<T, S: Storage> Handler<S> for TlvHandlerType<T, S> where T: TlvHandler<S> {
    fn handle(
        &mut self,
        uri: Uri,
        body: Vec<u8>,
        controller_id: Arc<Option<Uuid>>,
        database: &Arc<Mutex<Database<S>>>,
        accessory_list: &AccessoryList,
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
