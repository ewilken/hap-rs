use iron::prelude::{Request, Response, IronResult};
use iron::status;

pub fn pairings(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}
