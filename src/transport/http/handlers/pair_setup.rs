use std::io::{Read, Error};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::IpAddr;
use rand;
use rand::Rng;
use sha2::{Sha512, Digest};
use iron::prelude::{Request, Response, IronResult};
use iron::{status, Headers};
use srp::server::{UserRecord, SrpServer};
use srp::client::{SrpClient, SrpClientVerifier, srp_private_key};
use srp::groups::G_3072;
use serde_json;

use db::context::Context;
use transport::http::ContentType;
use transport::tlv;

pub fn pair_setup(request: &mut Request, context: &Arc<Mutex<Context>>) -> IronResult<Response> {
    let ip = Context::get_request_address(request).ip();

    let mut buf: Vec<u8> = Vec::new();
    request.body.by_ref().read_to_end(&mut buf).unwrap();

    let decoded = tlv::decode(buf);
    let mut answer: HashMap<u8, Vec<u8>> = HashMap::new();
    if let Some(v) = decoded.get(&0x06) {
        match v[0] {
            1 => {
                println!("/pair-setup - Got M1: SRP Start Request from {}", ip);
                let (t, v) = tlv::Type::State(2).as_type_value();
                answer.insert(t, v);

                // TODO - Errors for kTLVError_Unavailable, kTLVError_MaxTries and kTLVError_Busy

                let mut rng = rand::thread_rng();
                let salt = rng.gen_iter::<u8>().take(16).collect::<Vec<u8>>();
                let b = rng.gen_iter::<u8>().take(64).collect::<Vec<u8>>();

                let private_key = srp_private_key::<Sha512>(b"Pair-Setup", b"111-22-111", &salt);
                let srp_client = SrpClient::<Sha512>::new(&private_key, &G_3072);
                let verifier = srp_client.get_password_verifier(&private_key);

                let user = UserRecord {
                    username: b"Pair-Setup",
                    salt: &salt,
                    verifier: &verifier,
                };
                // TODO - return a kTLVError
                let srp_server = SrpServer::<Sha512>::new(&user, b"bar", &private_key, &G_3072).unwrap();

                let session = SrpPairingSession {
                    ip,
                    salt: salt.to_owned(),
                    verifier: verifier.to_owned(),
                    b: private_key.as_slice().to_vec(),
                    b_pub: srp_server.get_b_pub(),
                    next_step: 3,
                };

                let (t, v) = tlv::Type::PublicKey(session.b_pub.to_owned()).as_type_value();
                answer.insert(t, v);
                let (t, v) = tlv::Type::Salt(salt.to_owned()).as_type_value();
                answer.insert(t, v);

                // TODO - get rid of all those unwraps
                session.save(context).unwrap();

                println!("/pair-setup - Sending M2: SRP Start Response to {}", ip);
            },
            3 => {
                println!("/pair-setup - Got M3: SRP Verify Request from {}", ip);
                let (t, v) = tlv::Type::State(2).as_type_value();
                answer.insert(t, v);
                if let Some(session) = SrpPairingSession::load(ip, context) {
                    let a_pub = decoded.get(&0x03).unwrap();
                    let a_proof = decoded.get(&0x04).unwrap();
                    let user = UserRecord {
                        username: b"Pair-Setup",
                        salt: &session.salt,
                        verifier: &session.verifier,
                    };
                    let srp_server = SrpServer::<Sha512>::new(&user, &a_pub, &session.b, &G_3072).unwrap();
                    let shared_secret = srp_server.get_key();
                    let b_proof = srp_server.verify(a_proof).unwrap();
                    println!("{:?}", b_proof);

                    /*let mut rng = rand::thread_rng();
                    let a = rng.gen_iter::<u8>().take(64).collect::<Vec<u8>>();
                    let client = SrpClient::<Sha512>::new(&a, &G_3072);
                    let a_pub = client.get_a_pub();
                    let private_key = srp_private_key::<Sha512>(b"Pair-Setup", b"111-22-111", &session.salt);
                    let verifier = client.process_reply(&private_key, &session.b_pub).unwrap();
                    let test_a_proof = verifier.get_proof();
                    let test_server = SrpServer::<Sha512>::new(&user, &a_pub, &session.b, &G_3072).unwrap();
                    let test_b_proof = test_server.verify(&test_a_proof).unwrap();

                    println!("{:?}", test_b_proof);*/
                }

                //println!("/pair-setup - Sending M4: SRP Verify Response to {}", ip);
            },
            _ => {
                println!("/pair-setup - Got invalid state: M{} from {}", v[0], ip);
                let (t, v) = tlv::Type::State(0).as_type_value();
                answer.insert(t, v);
                // TODO - return a kTLVError?
            },
        }
    } else {
        let (t, v) = tlv::Type::State(0).as_type_value();
        answer.insert(t, v);
    }

    let body = tlv::encode(answer);

    let mut response = Response::with((status::Ok, body));
    response.headers.set_raw("Content-Type", vec![ContentType::PairingTLV8.as_vec()]);

    Ok(response)
}

#[derive(Serialize, Deserialize)]
struct SrpPairingSession {
    ip: IpAddr,
    salt: Vec<u8>,
    verifier: Vec<u8>,
    b: Vec<u8>,
    b_pub: Vec<u8>,
    next_step: u8,
}

impl SrpPairingSession {
    fn load(ip: IpAddr, context: &Arc<Mutex<Context>>) -> Option<SrpPairingSession> {
        let key = match ip {
            IpAddr::V4(addr) => addr.octets().to_vec(),
            IpAddr::V6(addr) => addr.octets().to_vec(),
        };
        let c = context.lock().unwrap();
        if let Some(val) = c.get(key) {
            return serde_json::from_slice(&val).ok();
        }
        None
    }

    fn save(&self, context: &Arc<Mutex<Context>>) -> Result<(), Error> {
        let key = match self.ip {
            IpAddr::V4(addr) => addr.octets().to_vec(),
            IpAddr::V6(addr) => addr.octets().to_vec(),
        };
        let val = serde_json::to_vec(self)?;
        let mut c = context.lock().unwrap();
        c.set(key, val);
        Ok(())
    }
}
