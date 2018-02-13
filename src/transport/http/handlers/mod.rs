use std::sync::{Arc, Mutex};
use futures::Future;
use hyper::{Uri, Error};
use hyper::server::Response;

use db::storage::Storage;
use db::database::Database;

pub mod accessories;
pub mod characteristics;
pub mod identify;
pub mod pair_setup;
pub mod pair_verify;
pub mod pairings;

pub trait Handler<S: Storage> {
    fn handle(&mut self, uri: Uri, body: Vec<u8>, database: &Arc<Mutex<Database<S>>>) -> Box<Future<Item=Response, Error=Error>>;
}
