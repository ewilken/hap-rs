use mdns;

use config;
use db;

pub struct IpTransport {
    config: config::Config,
    // TODO - generalize for any implementation of db::storage::Storage
    storage: db::storage::Storage,
    database: db::database::Database,
    responder: mdns::Responder,
}
