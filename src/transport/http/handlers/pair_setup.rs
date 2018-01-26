use std::io::Read;
use std::collections::HashMap;
use iron::prelude::{Request, Response, IronResult};
use iron::{status, Headers};

use protocol::context::Context;
use transport::http::ContentType;
use transport::tlv;

pub fn pair_setup(request: &mut Request) -> IronResult<Response> {
    let key = Context::get_request_address(request);

    let mut buf: Vec<u8> = Vec::new();
    request.body.by_ref().read_to_end(&mut buf).unwrap();
    println!("{:?}", buf);

    // TODO - Errors for kTLVError_Unavailable, kTLVError_MaxTries and kTLVError_Busy

    let mut map: HashMap<u8, Vec<u8>> = HashMap::new();
    let (t, v) = tlv::Type::State(2).as_type_value();
    map.insert(t, v);

    let body = tlv::encode(map);

    let mut response = Response::with((status::Ok, body));
    response.headers.set_raw("Content-Type", vec![ContentType::PairingTLV8.as_vec()]);

    Ok(response)
}
