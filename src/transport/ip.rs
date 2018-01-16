use mdns;

use config;
use db::{storage, database};

pub struct IpTransport<S: storage::Storage, D: storage::Storage> {
    config: config::Config,
    storage: S,
    database: database::Database<D>,
    responder: mdns::Responder,
}
