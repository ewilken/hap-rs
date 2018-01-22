use iron::Iron;

use transport::http::router;

pub fn serve() {
    let mut chain = router::chain();
    Iron::new(chain).http("120.0.0.1:8080".into()).unwrap();
}
