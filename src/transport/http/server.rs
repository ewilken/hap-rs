use std::net::SocketAddr;
use iron::Iron;

use transport::http::router;

pub fn serve(socket_addr: SocketAddr) {
    let mut chain = router::chain();
    Iron::new(chain).http(socket_addr).unwrap();
}
