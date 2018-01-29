use std::io::Read;
use std::collections::HashMap;
use rand;
use rand::Rng;
use sha2::{Sha256, Digest};
use iron::prelude::{Request, Response, IronResult};
use iron::{status, Headers};
use srp::server::{UserRecord, SrpServer};
use srp::groups::G_2048;

use protocol::context::Context;
use transport::http::ContentType;
use transport::tlv;

pub fn pair_setup(request: &mut Request) -> IronResult<Response> {
    let key = Context::get_request_address(request);

    let mut buf: Vec<u8> = Vec::new();
    request.body.by_ref().read_to_end(&mut buf).unwrap();

    let decoded = tlv::decode(buf);
    let mut answer: HashMap<u8, Vec<u8>> = HashMap::new();
    if let Some(v) = decoded.get(&0x06) {
        match v[0] {
            1 => {
                println!("Got M1: SRP Start Request");
                let (t, v) = tlv::Type::State(2).as_type_value();
                answer.insert(t, v);

                let mut rng = rand::thread_rng();
                let salt = rng.gen_iter::<u8>().take(16).collect::<Vec<u8>>();
                let b = rng.gen_iter::<u8>().take(64).collect::<Vec<u8>>();
                let user = UserRecord {
                    username: b"Pair-Setup",
                    salt: &salt,
                    verifier: b"111-22-333",
                };
                let srp_server = SrpServer::<Sha256>::new(&user, b"test", &b, &G_2048).unwrap();

                let (t, v) = tlv::Type::PublicKey(srp_server.get_b_pub()).as_type_value();
                answer.insert(t, v);
                let (t, v) = tlv::Type::Salt(salt.to_owned()).as_type_value();
                answer.insert(t, v);
                println!("Sending M2: SRP Start Response");
            },
            3 => {
                println!("Got M3: SRP Verify Request");

                println!("Sending M4: SRP Verify Response");
            },
            _ => {
                println!("Got invalid state: M{}", v[0]);
                let (t, v) = tlv::Type::State(0).as_type_value();
                answer.insert(t, v);
            },
        }
    } else {
        let (t, v) = tlv::Type::State(0).as_type_value();
        answer.insert(t, v);
    }

    // TODO - Errors for kTLVError_Unavailable, kTLVError_MaxTries and kTLVError_Busy

    let body = tlv::encode(answer);

    let mut response = Response::with((status::Ok, body));
    response.headers.set_raw("Content-Type", vec![ContentType::PairingTLV8.as_vec()]);

    Ok(response)
}
