use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use iron::Iron;

use transport::http::router;
use db::context::Context;
use db::storage::Storage;
use db::database::Database;
use config::Config;

pub fn serve<D: 'static + Storage + Send>(socket_addr: SocketAddr, config: Arc<Config>, context: Arc<Mutex<Context>>, database: Arc<Mutex<Database<D>>>) {
    let mut chain = router::chain(config, context, database);
    Iron::new(chain).http(socket_addr).unwrap();
}
