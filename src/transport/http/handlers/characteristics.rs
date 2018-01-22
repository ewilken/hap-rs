use iron::prelude::{Request, Response, IronResult};
use iron::status;

pub fn get_characteristics(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}

pub fn update_characteristics(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::Ok))
}
