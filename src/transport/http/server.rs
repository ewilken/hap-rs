use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use iron::Iron;

use transport::http::router;
use db::context::Context;
use config::Config;

pub fn serve(socket_addr: SocketAddr, config: Arc<Config>, context: Arc<Mutex<Context>>) {
    let mut chain = router::chain(config, context);
    Iron::new(chain).http(socket_addr).unwrap();
}
