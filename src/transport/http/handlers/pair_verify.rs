use std::io::{Read, Error, ErrorKind};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use iron::prelude::{Request, Response, IronResult};
use rand::{self, Rng};
use crypto::{curve25519, ed25519};
use ring::{hkdf, hmac, digest};
use chacha20_poly1305_aead;

use transport::http::{response, ContentType};
use transport::tlv;
use db::context::Context;
use db::storage::Storage;
use db::database::Database;
use config::Config;
use protocol::device::Device;

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

                if let Some(curve_a_pub) = decoded.get(&0x03) {
                    let mut rng = rand::thread_rng();
                    let curve_b = rng.gen::<[u8; 32]>();
                    let curve_b_pub = curve25519::ge_scalarmult_base(&curve_b).to_bytes();
                    let curve_shared_secret = curve25519::curve25519(&curve_b, &curve_a_pub);

                    let accessory = Device::load::<D>(context, database).unwrap();
                    let mut accessory_info: Vec<u8> = Vec::new();
                    accessory_info.extend(&curve_b_pub);
                    accessory_info.extend(accessory.id.as_bytes());
                    accessory_info.extend(curve_a_pub);
                    let accessory_signature = ed25519::signature(&accessory_info, &accessory.private_key);

                    let mut sub_tlv: HashMap<u8, Vec<u8>> = HashMap::new();
                    let (t, v) = tlv::Type::Identifier(accessory.id).as_type_value();
                    sub_tlv.insert(t, v);
                    let (t, v) = tlv::Type::Signature(accessory_signature.to_vec()).as_type_value();
                    sub_tlv.insert(t, v);
                    let encoded_sub_tlv = tlv::encode(sub_tlv);

                    let mut encryption_key = vec![0; 32];
                    let salt = hmac::SigningKey::new(&digest::SHA512, b"Pair-Verify-Encrypt-Salt");
                    hkdf::extract_and_expand(&salt, &curve_shared_secret, b"Pair-Verify-Encrypt-Info", &mut encryption_key);

                    let mut encrypted_data = Vec::new();
                    let mut nonce = vec![0, 0, 0, 0];
                    nonce.extend(b"PV-Msg02");
                    let auth_tag = chacha20_poly1305_aead::encrypt(&encryption_key, &nonce, &[], &encoded_sub_tlv, &mut encrypted_data).unwrap();
                    encrypted_data.extend(&auth_tag);

                    let (t, v) = tlv::Type::PublicKey(curve_b_pub.to_vec()).as_type_value();
                    answer.insert(t, v);
                    let (t, v) = tlv::Type::EncryptedData(encrypted_data).as_type_value();
                    answer.insert(t, v);

                    println!("/pair-verify - M2: Sending Verify Start Response to {}", ip);
                }
            },
            3 => {
                println!("/pair-verify - M3: Got Verify Finish Request from {}", ip);
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
