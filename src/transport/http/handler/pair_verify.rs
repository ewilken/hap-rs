use std::str;

use aead::{generic_array::GenericArray, Aead, NewAead};
use chacha20poly1305::ChaCha20Poly1305;
use futures::{
    channel::oneshot,
    future::{BoxFuture, FutureExt},
    stream::StreamExt,
};
use hyper::Body;
use log::{debug, info};
use rand::{self, rngs::OsRng};
use ring::{digest, hkdf, hmac};
use uuid::Uuid;
use x25519_dalek::{EphemeralSecret, PublicKey};

use crate::{
    pointer,
    tlv::{self, Encodable, Type, Value},
    transport::{http::handler::TlvHandlerExt, tcp},
};

struct Session {
    b_pub: PublicKey,
    a_pub: PublicKey,
    shared_secret: [u8; 32],
    session_key: [u8; 32],
}

pub struct PairVerify {
    session: Option<Session>,
    session_sender: Option<oneshot::Sender<tcp::Session>>,
}

impl PairVerify {
    pub fn new(session_sender: oneshot::Sender<tcp::Session>) -> PairVerify {
        PairVerify {
            session: None,
            session_sender: Some(session_sender),
        }
    }
}

#[derive(Debug, Clone)]
enum StepNumber {
    Unknown = 0,
    StartReq = 1,
    StartRes = 2,
    FinishReq = 3,
    FinishRes = 4,
}

#[derive(Debug, Clone)]
pub enum Step {
    Start { a_pub: Vec<u8> },
    Finish { data: Vec<u8> },
}

impl TlvHandlerExt for PairVerify {
    type ParseResult = Step;
    type Result = tlv::Container;

    fn parse(&self, body: Body) -> BoxFuture<Result<Step, tlv::ErrorContainer>> {
        async {
            let mut body = body;
            let mut concatenated_body = Vec::new();
            while let Some(chunk) = body.next().await {
                let bytes =
                    chunk.map_err(|_| tlv::ErrorContainer::new(StepNumber::Unknown as u8, tlv::Error::Unknown))?;
                concatenated_body.extend(&bytes[..]);
            }

            debug!("received body: {:?}", &concatenated_body);

            let mut decoded = tlv::decode(concatenated_body);
            match decoded.get(&(Type::State as u8)) {
                Some(method) => match method[0] {
                    x if x == StepNumber::StartReq as u8 => {
                        let a_pub = decoded
                            .remove(&(Type::PublicKey as u8))
                            .ok_or(tlv::ErrorContainer::new(
                                StepNumber::StartRes as u8,
                                tlv::Error::Unknown,
                            ))?;
                        Ok(Step::Start { a_pub })
                    },
                    x if x == StepNumber::FinishReq as u8 => {
                        let data = decoded
                            .remove(&(Type::EncryptedData as u8))
                            .ok_or(tlv::ErrorContainer::new(
                                StepNumber::FinishRes as u8,
                                tlv::Error::Unknown,
                            ))?;
                        Ok(Step::Finish { data })
                    },
                    _ => Err(tlv::ErrorContainer::new(StepNumber::Unknown as u8, tlv::Error::Unknown)),
                },
                None => Err(tlv::ErrorContainer::new(StepNumber::Unknown as u8, tlv::Error::Unknown)),
            }
        }
        .boxed()
    }

    fn handle(
        &mut self,
        step: Step,
        _: pointer::ControllerId,
        config: pointer::Config,
        storage: pointer::Storage,
        _: pointer::EventEmitter,
    ) -> BoxFuture<Result<tlv::Container, tlv::ErrorContainer>> {
        async move {
            match step {
                Step::Start { a_pub } => match handle_start(self, config, a_pub).await {
                    Ok(res) => Ok(res),
                    Err(err) => Err(tlv::ErrorContainer::new(StepNumber::StartRes as u8, err)),
                },
                Step::Finish { data } => match handle_finish(self, storage, &data).await {
                    Ok(res) => Ok(res),
                    Err(err) => Err(tlv::ErrorContainer::new(StepNumber::FinishRes as u8, err)),
                },
            }
        }
        .boxed()
    }
}

async fn handle_start(
    handler: &mut PairVerify,
    config: pointer::Config,
    a_pub_bytes: Vec<u8>,
) -> Result<tlv::Container, tlv::Error> {
    info!("pair verify M1: received verify start request");

    // let mut rng = rand::thread_rng();
    // let b = rng.gen::<[u8; 32]>();
    // let b_pub = curve25519::curve25519_base(&b);
    // let shared_secret = curve25519::curve25519(b, a_pub);

    let mut a_pub = [0; 32];
    let bytes = &a_pub_bytes[..a_pub.len()]; // panics if not enough data
    a_pub.copy_from_slice(bytes);
    let a_pub = PublicKey::from(a_pub);

    let mut csprng = OsRng {};
    let b = EphemeralSecret::new(&mut csprng);
    let b_pub = PublicKey::from(&b);
    let shared_secret = b.diffie_hellman(&a_pub);

    let config = config.lock().expect("couldn't access config");
    let device_id = config.device_id.to_hex_string();

    let mut accessory_info: Vec<u8> = Vec::new();
    accessory_info.extend(b_pub.as_bytes());
    accessory_info.extend(device_id.as_bytes());
    accessory_info.extend(a_pub.as_bytes());
    let accessory_signature = config.device_ed25519_keypair.sign(&accessory_info);

    let encoded_sub_tlv = vec![Value::Identifier(device_id), Value::Signature(accessory_signature)].encode();

    let mut session_key = [0; 32];
    let salt = hmac::SigningKey::new(&digest::SHA512, b"Pair-Verify-Encrypt-Salt");
    hkdf::extract_and_expand(
        &salt,
        shared_secret.as_bytes(),
        b"Pair-Verify-Encrypt-Info",
        &mut session_key,
    );

    handler.session = Some(Session {
        b_pub,
        a_pub,
        shared_secret: *shared_secret.as_bytes(),
        session_key,
    });

    let mut nonce = vec![0; 4];
    nonce.extend(b"PV-Msg02");

    let aead = ChaCha20Poly1305::new(session_key.into());

    let mut encrypted_data = Vec::new();
    encrypted_data.extend_from_slice(&encoded_sub_tlv);
    let auth_tag = aead.encrypt_in_place_detached(GenericArray::from_slice(&nonce), &[], &mut encrypted_data)?;
    encrypted_data.extend(&auth_tag);

    info!("pair verify M2: sending verify start response");

    Ok(vec![
        Value::State(StepNumber::StartRes as u8),
        Value::X25519PublicKey(b_pub),
        Value::EncryptedData(encrypted_data),
    ])
}

async fn handle_finish(
    handler: &mut PairVerify,
    storage: pointer::Storage,
    data: &[u8],
) -> Result<tlv::Container, tlv::Error> {
    info!("pair verify M3: received verify finish request");

    if let Some(ref mut session) = handler.session {
        let encrypted_data = Vec::from(&data[..data.len() - 16]);
        let auth_tag = Vec::from(&data[data.len() - 16..]);

        let mut nonce = vec![0; 4];
        nonce.extend(b"PV-Msg03");

        let aead = ChaCha20Poly1305::new(session.session_key.into());

        let mut decrypted_data = Vec::new();
        decrypted_data.extend_from_slice(&encrypted_data);
        aead.decrypt_in_place_detached(
            GenericArray::from_slice(&nonce),
            &[],
            &mut decrypted_data,
            GenericArray::from_slice(&auth_tag),
        )?;

        let sub_tlv = tlv::decode(decrypted_data);
        let device_pairing_id = sub_tlv.get(&(Type::Identifier as u8)).ok_or(tlv::Error::Unknown)?;
        let device_signature =
            ed25519_dalek::Signature::from_bytes(sub_tlv.get(&(Type::Signature as u8)).ok_or(tlv::Error::Unknown)?)?;

        let uuid_str = str::from_utf8(device_pairing_id)?;
        let pairing_uuid = Uuid::parse_str(uuid_str)?;
        let pairing = storage
            .lock()
            .expect("couldn't access storage")
            .select_pairing(&pairing_uuid)?;

        let mut device_info: Vec<u8> = Vec::new();
        device_info.extend(session.a_pub.as_bytes());
        device_info.extend(device_pairing_id);
        device_info.extend(session.b_pub.as_bytes());

        // if !ed25519::verify(&device_info, &pairing.public_key, &device_signature) {
        //     return Err(tlv::Error::Authentication);
        // }
        if pairing.public_key.verify(&device_info, &device_signature).is_err() {
            return Err(tlv::Error::Authentication);
        }

        if let Some(sender) = handler.session_sender.take() {
            let encrypted_session = tcp::Session {
                controller_id: pairing_uuid,
                shared_secret: session.shared_secret,
            };
            let _session = sender.send(encrypted_session);
        } else {
            return Err(tlv::Error::Unknown);
        }

        info!("pair verify M4: sending verify finish response");

        Ok(vec![Value::State(StepNumber::FinishRes as u8)])
    } else {
        Err(tlv::Error::Unknown)
    }
}
