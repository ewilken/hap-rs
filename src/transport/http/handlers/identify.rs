use iron::prelude::{Request, Response, IronResult};
use iron::status;

pub fn identify(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}
