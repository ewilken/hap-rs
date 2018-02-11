use std::io::{Read, Error};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::net::IpAddr;
use std::str;
use iron::prelude::{Request, Response, IronResult};
use rand::{self, Rng};
use crypto::{curve25519, ed25519};
use ring::{hkdf, hmac, digest};
use chacha20_poly1305_aead;
use serde_json;
use uuid::Uuid;

use transport::http::{response, ContentType};
use transport::tlv;
use db::context::Context;
use db::storage::Storage;
use db::database::Database;
use config::Config;
use protocol::device::Device;
use protocol::pairing::Pairing;

pub fn pair_verify<D: Storage + Send>(request: &mut Request, config: &Arc<Config>, context: &Arc<Mutex<Context>>, database: &Arc<Mutex<Database<D>>>) -> IronResult<Response> {
    let ip = Context::get_request_address(request).ip();

    let mut buf: Vec<u8> = Vec::new();
    request.body.by_ref().read_to_end(&mut buf).unwrap();

    let decoded = tlv::decode(buf);
    let mut answer: HashMap<u8, Vec<u8>> = HashMap::new();

    if let Some(v) = decoded.get(&0x06) {
        match v[0] {
            1 => {
                println!("/pair-verify - M1: Got Verify Start Request from {}", ip);

                let (t, v) = tlv::Type::State(2).as_type_value();
                answer.insert(t, v);

                if let Some(a_pub) = decoded.get(&0x03) {
                    let mut rng = rand::thread_rng();
                    let b = rng.gen::<[u8; 32]>();
                    let b_pub = curve25519::curve25519_base(&b);
                    let shared_secret = curve25519::curve25519(&b, &a_pub);

                    let accessory = Device::load::<D>(context, database).unwrap();
                    let mut accessory_info: Vec<u8> = Vec::new();
                    accessory_info.extend(&b_pub);
                    accessory_info.extend(accessory.id.as_bytes());
                    accessory_info.extend(a_pub);
                    let accessory_signature = ed25519::signature(&accessory_info, &accessory.private_key);

                    let mut sub_tlv: HashMap<u8, Vec<u8>> = HashMap::new();
                    let (t, v) = tlv::Type::Identifier(accessory.id).as_type_value();
                    sub_tlv.insert(t, v);
                    let (t, v) = tlv::Type::Signature(accessory_signature.to_vec()).as_type_value();
                    sub_tlv.insert(t, v);
                    let encoded_sub_tlv = tlv::encode(sub_tlv);

                    let mut session_key = [0; 32];
                    let salt = hmac::SigningKey::new(&digest::SHA512, b"Pair-Verify-Encrypt-Salt");
                    hkdf::extract_and_expand(&salt, &shared_secret, b"Pair-Verify-Encrypt-Info", &mut session_key);
                    
                    let session = PairVerificationSession {
                        ip,
                        b,
                        b_pub,
                        a_pub: a_pub.to_owned(),
                        shared_secret,
                        session_key,
                    };

                    let mut encrypted_data = Vec::new();
                    let mut nonce = vec![0, 0, 0, 0];
                    nonce.extend(b"PV-Msg02");
                    let auth_tag = chacha20_poly1305_aead::encrypt(&session_key, &nonce, &[], &encoded_sub_tlv, &mut encrypted_data).unwrap();
                    encrypted_data.extend(&auth_tag);

                    let (t, v) = tlv::Type::PublicKey(b_pub.to_vec()).as_type_value();
                    answer.insert(t, v);
                    let (t, v) = tlv::Type::EncryptedData(encrypted_data).as_type_value();
                    answer.insert(t, v);
                    
                    session.save(context).unwrap();

                    println!("/pair-verify - M2: Sending Verify Start Response to {}", ip);
                }
            },
            3 => {
                println!("/pair-verify - M3: Got Verify Finish Request from {}", ip);
                
                let (t, v) = tlv::Type::State(4).as_type_value();
                answer.insert(t, v);
                
                if let Some(mut session) = PairVerificationSession::load(ip, context) {
                    let data = decoded.get(&0x05).unwrap();
                    let encrypted_data = Vec::from(&data[..data.len() - 16]);
                    let auth_tag = Vec::from(&data[data.len() - 16..]);
                    
                    let mut decrypted_data = Vec::new();
                    let mut nonce = vec![0, 0, 0, 0];
                    nonce.extend(b"PV-Msg03");
                    chacha20_poly1305_aead::decrypt(&session.session_key, &nonce, &[], &encrypted_data, &auth_tag, &mut decrypted_data).unwrap();
                    
                    let sub_tlv = tlv::decode(decrypted_data);
                    let device_pairing_id = sub_tlv.get(&0x01).unwrap();
                    let device_signature = sub_tlv.get(&0x0A).unwrap();
                    
                    let uuid_str = str::from_utf8(device_pairing_id).unwrap();
                    let pairing_uuid = Uuid::parse_str(uuid_str).unwrap();
                    let pairing = Pairing::load::<D>(pairing_uuid, context, database).unwrap();
                    
                    let mut device_info: Vec<u8> = Vec::new();
                    device_info.extend(&session.a_pub);
                    device_info.extend(device_pairing_id);
                    device_info.extend(&session.b_pub);
                    if !ed25519::verify(&device_info, &pairing.public_key, &device_signature) {
                        let (t, v) = tlv::Type::Error(tlv::ErrorKind::Authentication).as_type_value();
                        answer.insert(t, v);
                        return Ok(response(answer, ContentType::PairingTLV8));
                    }
                    
                    println!("/pair-verify - M4: Sending Verify Finish Response to {}", ip);
                }
            },
            _ => {
                println!("/pair-verify - M{}: Got invalid state from {}", v[0], ip);
                let (t, v) = tlv::Type::State(0).as_type_value();
                answer.insert(t, v);
                // TODO - return a kTLVError?
            },
        }
    }

    Ok(response(answer, ContentType::PairingTLV8))
}

#[derive(Serialize, Deserialize)]
struct PairVerificationSession {
    ip: IpAddr,
    b: [u8; 32],
    b_pub: [u8; 32],
    a_pub: Vec<u8>,
    shared_secret: [u8; 32],
    session_key: [u8; 32],
}

impl PairVerificationSession {
    fn load(ip: IpAddr, context: &Arc<Mutex<Context>>) -> Option<PairVerificationSession> {
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
