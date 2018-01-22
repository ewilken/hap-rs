use iron::prelude::{Request, Response, IronResult};
use iron::status;

use protocol::context::Context;

pub fn pair_setup(req: &mut Request) -> IronResult<Response> {
    let key = Context::get_connection_key(req);

    Ok(Response::with(status::Ok))
}
