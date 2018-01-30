use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use iron::Iron;

use transport::http::router;
use db::context::Context;

pub fn serve(socket_addr: SocketAddr, context: Arc<Mutex<Context>>) {
    let mut chain = router::chain(context);
    Iron::new(chain).http(socket_addr).unwrap();
}
