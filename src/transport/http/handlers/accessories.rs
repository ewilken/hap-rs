use iron::prelude::{Request, Response, IronResult};
use iron::status;

pub fn accessories(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}
