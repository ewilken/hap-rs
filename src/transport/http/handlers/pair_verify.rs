use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::str;
use rand::{self, Rng};
use crypto::{curve25519, ed25519};
use ring::{hkdf, hmac, digest};
use chacha20_poly1305_aead;
use uuid::Uuid;
use futures::sync::oneshot;

use transport::http::handlers::TlvHandler;
use transport::http::encrypted_stream;
use transport::tlv::{self, Type, Value};
use config::Config;
use db::storage::Storage;
use db::database::Database;
use protocol::device::Device;
use protocol::pairing::Pairing;

struct Session {
    b_pub: [u8; 32],
    a_pub: Vec<u8>,
    shared_secret: [u8; 32],
    session_key: [u8; 32],
}

pub struct PairVerify {
    session: Option<Session>,
    session_sender: Option<oneshot::Sender<encrypted_stream::Session>>,
}

impl PairVerify {
    pub fn new(session_sender: oneshot::Sender<encrypted_stream::Session>) -> PairVerify {
        PairVerify { session: None, session_sender: Some(session_sender) }
    }
}

pub enum Step {
    Start { a_pub: Vec<u8> },
    Finish { data: Vec<u8> },
}

impl<S: Storage> TlvHandler<S> for PairVerify {
    type ParseResult = Step;
    type Result = tlv::Container;

    fn parse(&self, body: Vec<u8>) -> Result<Step, tlv::ErrorContainer> {
        let decoded = tlv::decode(body);
        match decoded.get(&(Type::State as u8)) {
            Some(method) => match method[0] {
                // TODO - put those step numbers into the step enum somehow
                1 => {
                    let a_pub = decoded.get(&(Type::PublicKey as u8)).ok_or(
                        tlv::ErrorContainer::new(2, tlv::Error::Unknown)
                    )?;
                    Ok(Step::Start { a_pub: a_pub.clone() })
                },
                3 => {
                    let data = decoded.get(&(Type::EncryptedData as u8)).ok_or(
                        tlv::ErrorContainer::new(4, tlv::Error::Unknown)
                    )?;
                    Ok(Step::Finish { data: data.clone() })
                },
                _ => Err(tlv::ErrorContainer::new(0, tlv::Error::Unknown)),
            },
            None => Err(tlv::ErrorContainer::new(0, tlv::Error::Unknown)),
        }
    }

    fn handle(
        &mut self,
        step: Step,
        database: &Arc<Mutex<Database<S>>>
    ) -> Result<tlv::Container, tlv::ErrorContainer> {
        match step {
            Step::Start { a_pub } => match handle_start::<S>(self, database, a_pub) {
                Ok(res) => Ok(res),
                Err(err) => Err(tlv::ErrorContainer::new(2, err)),
            },
            Step::Finish { data } => match handle_finish::<S>(self, database, data) {
                Ok(res) => Ok(res),
                Err(err) => Err(tlv::ErrorContainer::new(4, err)),
            },
        }
    }
}

fn handle_start<S: Storage>(
    handler: &mut PairVerify,
    database: &Arc<Mutex<Database<S>>>,
    a_pub: Vec<u8>,
) -> Result<tlv::Container, tlv::Error> {
    let mut rng = rand::thread_rng();
    let b = rng.gen::<[u8; 32]>();
    let b_pub = curve25519::curve25519_base(&b);
    let shared_secret = curve25519::curve25519(&b, &a_pub);

    let accessory = Device::load::<S>(database)?;
    let mut accessory_info: Vec<u8> = Vec::new();
    accessory_info.extend(&b_pub);
    accessory_info.extend(accessory.id.as_bytes());
    accessory_info.extend(&a_pub);
    let accessory_signature = ed25519::signature(&accessory_info, &accessory.private_key);

    let mut sub_tlv: HashMap<u8, Vec<u8>> = HashMap::new();
    let (t, v) = Value::Identifier(accessory.id).as_tlv();
    sub_tlv.insert(t, v);
    let (t, v) = Value::Signature(accessory_signature.to_vec()).as_tlv();
    sub_tlv.insert(t, v);
    let encoded_sub_tlv = tlv::encode(sub_tlv);

    let mut session_key = [0; 32];
    let salt = hmac::SigningKey::new(&digest::SHA512, b"Pair-Verify-Encrypt-Salt");
    hkdf::extract_and_expand(&salt, &shared_secret, b"Pair-Verify-Encrypt-Info", &mut session_key);

    handler.session = Some(Session {
        b_pub,
        a_pub,
        shared_secret,
        session_key,
    });

    let mut encrypted_data = Vec::new();
    let mut nonce = vec![0; 4];
    nonce.extend(b"PV-Msg02");
    let auth_tag = chacha20_poly1305_aead::encrypt(
        &session_key,
        &nonce, &[],
        &encoded_sub_tlv,
        &mut encrypted_data,
    )?;
    encrypted_data.extend(&auth_tag);

    Ok(vec![
        Value::State(2),
        Value::PublicKey(b_pub.to_vec()),
        Value::EncryptedData(encrypted_data),
    ])
}

fn handle_finish<S: Storage>(
    handler: &mut PairVerify,
    database: &Arc<Mutex<Database<S>>>,
    data: Vec<u8>,
) -> Result<tlv::Container, tlv::Error> {
    if let Some(ref mut session) = handler.session {
        let encrypted_data = Vec::from(&data[..data.len() - 16]);
        let auth_tag = Vec::from(&data[data.len() - 16..]);

        let mut decrypted_data = Vec::new();
        let mut nonce = vec![0; 4];
        nonce.extend(b"PV-Msg03");
        chacha20_poly1305_aead::decrypt(
            &session.session_key,
            &nonce,
            &[],
            &encrypted_data,
            &auth_tag,
            &mut decrypted_data
        )?;

        let sub_tlv = tlv::decode(decrypted_data);
        let device_pairing_id = sub_tlv.get(&(Type::Identifier as u8)).ok_or(tlv::Error::Unknown)?;
        let device_signature = sub_tlv.get(&(Type::Signature as u8)).ok_or(tlv::Error::Unknown)?;

        let uuid_str = str::from_utf8(device_pairing_id)?;
        let pairing_uuid = Uuid::parse_str(uuid_str)?;
        let pairing = Pairing::load::<S>(pairing_uuid, database)?;

        let mut device_info: Vec<u8> = Vec::new();
        device_info.extend(&session.a_pub);
        device_info.extend(device_pairing_id);
        device_info.extend(&session.b_pub);
        if !ed25519::verify(&device_info, &pairing.public_key, &device_signature) {
            return Err(tlv::Error::Authentication);
        }

        if let Some(sender) = handler.session_sender.take() {
            let encrypted_session = encrypted_stream::Session {
                controller_id: pairing_uuid,
                shared_secret: session.shared_secret,
            };
            sender.send(encrypted_session);
        } else {
            return Err(tlv::Error::Unknown);
        }

        Ok(vec![Value::State(4)])
    } else {
        Err(tlv::Error::Unknown)
    }
}
