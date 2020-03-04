use std::{ops::BitXor, str};

use aead::{generic_array::GenericArray, Aead, NewAead};
use chacha20poly1305::ChaCha20Poly1305;
use futures::{
    future::{BoxFuture, FutureExt},
    stream::StreamExt,
};
use hyper::Body;
use log::{debug, info};
use num::BigUint;
use rand::{rngs::OsRng, RngCore};
use ring::{digest, hkdf, hmac};
use sha2::{Digest, Sha512};
use srp::{
    client::{srp_private_key, SrpClient},
    groups::G_3072,
    server::{SrpServer, UserRecord},
    types::SrpGroup,
};
use uuid::Uuid;
use x25519_dalek::PublicKey;

use crate::{
    event::Event,
    pairing::{Pairing, Permissions},
    pointer,
    tlv::{self, Encodable, Type, Value},
    transport::http::handler::TlvHandlerExt,
};

struct Session {
    salt: [u8; 16],
    verifier: Vec<u8>,
    b: [u8; 64],
    b_pub: Vec<u8>,
    shared_secret: Option<Vec<u8>>,
}

pub struct PairSetup {
    session: Option<Session>,
    unsuccessful_tries: u8,
}

impl PairSetup {
    pub fn new() -> PairSetup {
        PairSetup {
            session: None,
            unsuccessful_tries: 0,
        }
    }
}

#[derive(Debug, Clone)]
enum StepNumber {
    Unknown = 0,
    StartReq = 1,
    StartRes = 2,
    VerifyReq = 3,
    VerifyRes = 4,
    ExchangeReq = 5,
    ExchangeRes = 6,
}

#[derive(Debug, Clone)]
pub enum Step {
    Start,
    Verify { a_pub: Vec<u8>, a_proof: Vec<u8> },
    Exchange { data: Vec<u8> },
}

impl TlvHandlerExt for PairSetup {
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
                    x if x == StepNumber::StartReq as u8 => Ok(Step::Start),
                    x if x == StepNumber::VerifyReq as u8 => {
                        let a_pub = decoded
                            .remove(&(Type::PublicKey as u8))
                            .ok_or(tlv::ErrorContainer::new(
                                StepNumber::VerifyRes as u8,
                                tlv::Error::Unknown,
                            ))?;
                        let a_proof = decoded.remove(&(Type::Proof as u8)).ok_or(tlv::ErrorContainer::new(
                            StepNumber::VerifyRes as u8,
                            tlv::Error::Unknown,
                        ))?;
                        Ok(Step::Verify { a_pub, a_proof })
                    },
                    x if x == StepNumber::ExchangeReq as u8 => {
                        let data = decoded
                            .remove(&(Type::EncryptedData as u8))
                            .ok_or(tlv::ErrorContainer::new(
                                StepNumber::ExchangeRes as u8,
                                tlv::Error::Unknown,
                            ))?;
                        Ok(Step::Exchange { data })
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
        event_emitter: pointer::EventEmitter,
    ) -> BoxFuture<Result<tlv::Container, tlv::ErrorContainer>> {
        async move {
            match step {
                Step::Start => match handle_start(self, config).await {
                    Ok(res) => {
                        self.unsuccessful_tries = 0;
                        Ok(res)
                    },
                    Err(err) => {
                        self.unsuccessful_tries += 1;
                        Err(tlv::ErrorContainer::new(StepNumber::StartRes as u8, err))
                    },
                },
                Step::Verify { a_pub, a_proof } => match handle_verify(self, &a_pub, &a_proof).await {
                    Ok(res) => {
                        self.unsuccessful_tries = 0;
                        Ok(res)
                    },
                    Err(err) => {
                        self.unsuccessful_tries += 1;
                        Err(tlv::ErrorContainer::new(StepNumber::VerifyRes as u8, err))
                    },
                },
                Step::Exchange { data } => match handle_exchange(self, config, storage, event_emitter, &data).await {
                    Ok(res) => {
                        self.unsuccessful_tries = 0;
                        Ok(res)
                    },
                    Err(err) => {
                        self.unsuccessful_tries += 1;
                        Err(tlv::ErrorContainer::new(StepNumber::ExchangeRes as u8, err))
                    },
                },
            }
        }
        .boxed()
    }
}

async fn handle_start(handler: &mut PairSetup, config: pointer::Config) -> Result<tlv::Container, tlv::Error> {
    info!("pair setup M1: received SRP start request");

    if handler.unsuccessful_tries > 99 {
        return Err(tlv::Error::MaxTries);
    }

    // let rng = rand::thread_rng();
    // let salt = rng.sample_iter::<u8, Standard>(Standard).take(16).collect::<Vec<u8>>(); // s
    // let b = rng.sample_iter::<u8, Standard>(Standard).take(64).collect::<Vec<u8>>();

    let mut csprng = OsRng {};
    let mut salt = [0; 16]; // s
    let mut b = [0; 64];
    csprng.fill_bytes(&mut salt);
    csprng.fill_bytes(&mut b);

    let private_key = srp_private_key::<Sha512>(
        b"Pair-Setup",
        config.lock().expect("couldn't access config").pin.as_bytes(),
        &salt,
    ); // x = H(s | H(I | ":" | P))
    let srp_client = SrpClient::<Sha512>::new(&private_key, &G_3072);
    let verifier = srp_client.get_password_verifier(&private_key); // v = g^x

    let user = UserRecord {
        username: b"Pair-Setup",
        salt: &salt,
        verifier: &verifier,
    };
    let srp_server = SrpServer::<Sha512>::new(&user, b"foo", &b, &G_3072)?;
    let b_pub_bytes = srp_server.get_b_pub();

    let mut b_pub = [0; 32];
    let bytes = &b_pub_bytes[..b_pub.len()]; // panics if not enough data
    b_pub.copy_from_slice(bytes);
    let b_pub = PublicKey::from(b_pub);

    handler.session = Some(Session {
        salt,
        verifier,
        b,
        b_pub: b_pub_bytes.clone(),
        shared_secret: None,
    });

    info!("pair setup M2: sending SRP start response");

    Ok(vec![
        Value::State(StepNumber::StartRes as u8),
        // Value::X25519PublicKey(b_pub),
        Value::PublicKey(b_pub_bytes),
        Value::Salt(salt),
    ])
}

async fn handle_verify(handler: &mut PairSetup, a_pub: &[u8], a_proof: &[u8]) -> Result<tlv::Container, tlv::Error> {
    info!("pair setup M3: received SRP verify request");

    if let Some(ref mut session) = handler.session {
        let user = UserRecord {
            username: b"Pair-Setup",
            salt: &session.salt,
            verifier: &session.verifier,
        };
        let srp_server = SrpServer::<Sha512>::new(&user, &a_pub, &session.b, &G_3072)?;
        let shared_secret = srp_server.get_key();
        session.shared_secret = Some(shared_secret.as_slice().to_vec());
        let b_proof = verify_client_proof::<Sha512>(
            &session.b_pub,
            a_pub,
            a_proof,
            &session.salt,
            &shared_secret.as_slice().to_vec(),
            &G_3072,
        )?;

        info!("pair setup M4: sending SRP verify response");

        Ok(vec![Value::State(StepNumber::VerifyRes as u8), Value::Proof(b_proof)])
    } else {
        Err(tlv::Error::Unknown)
    }
}

async fn handle_exchange(
    handler: &mut PairSetup,
    config: pointer::Config,
    storage: pointer::Storage,
    event_emitter: pointer::EventEmitter,
    data: &[u8],
) -> Result<tlv::Container, tlv::Error> {
    info!("pair setup M5: received SRP exchange request");

    if let Some(ref mut session) = handler.session {
        if let Some(ref mut shared_secret) = session.shared_secret {
            let encrypted_data = Vec::from(&data[..data.len() - 16]);
            let auth_tag = Vec::from(&data[data.len() - 16..]);

            let mut encryption_key = [0; 32];
            let salt = hmac::SigningKey::new(&digest::SHA512, b"Pair-Setup-Encrypt-Salt");
            hkdf::extract_and_expand(&salt, &shared_secret, b"Pair-Setup-Encrypt-Info", &mut encryption_key);

            let mut nonce = vec![0; 4];
            nonce.extend(b"PS-Msg05");

            let aead = ChaCha20Poly1305::new(encryption_key.into());

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
            let device_ltpk = ed25519_dalek::PublicKey::from_bytes(
                sub_tlv.get(&(Type::PublicKey as u8)).ok_or(tlv::Error::Unknown)?,
            )?;
            let device_signature = ed25519_dalek::Signature::from_bytes(
                sub_tlv.get(&(Type::Signature as u8)).ok_or(tlv::Error::Unknown)?,
            )?;

            let mut device_x = [0; 32];
            let salt = hmac::SigningKey::new(&digest::SHA512, b"Pair-Setup-Controller-Sign-Salt");
            hkdf::extract_and_expand(&salt, &shared_secret, b"Pair-Setup-Controller-Sign-Info", &mut device_x);

            let mut device_info: Vec<u8> = Vec::new();
            device_info.extend(&device_x);
            device_info.extend(device_pairing_id);
            device_info.extend(device_ltpk.as_bytes());

            // if !ed25519::verify(&device_info, &device_ltpk, &device_signature) {
            //     return Err(tlv::Error::Authentication);
            // }
            if device_ltpk.verify(&device_info, &device_signature).is_err() {
                return Err(tlv::Error::Authentication);
            }

            let uuid_str = str::from_utf8(device_pairing_id)?;
            let pairing_uuid = Uuid::parse_str(uuid_str)?;
            let mut pairing_ltpk = [0; 32];
            pairing_ltpk[..32].clone_from_slice(&device_ltpk.as_bytes()[..32]);

            if let Some(max_peers) = config.lock().expect("couldn't access config").max_peers {
                if storage.lock().expect("couldn't access storage").count_pairings()? + 1 > max_peers {
                    return Err(tlv::Error::MaxPeers);
                }
            }

            let pairing = Pairing::new(pairing_uuid, Permissions::Admin, device_ltpk);
            storage
                .lock()
                .expect("couldn't access storage")
                .insert_pairing(&pairing)?;

            let mut accessory_x = [0; 32];
            let salt = hmac::SigningKey::new(&digest::SHA512, b"Pair-Setup-Accessory-Sign-Salt");
            hkdf::extract_and_expand(
                &salt,
                &shared_secret,
                b"Pair-Setup-Accessory-Sign-Info",
                &mut accessory_x,
            );

            let config = config.lock().expect("couldn't access config");
            let device_id = config.device_id.to_hex_string();

            let mut accessory_info: Vec<u8> = Vec::new();
            accessory_info.extend(&accessory_x);
            accessory_info.extend(device_id.as_bytes());
            accessory_info.extend(config.device_ed25519_keypair.public.as_bytes());
            let accessory_signature = config.device_ed25519_keypair.sign(&accessory_info);

            let encoded_sub_tlv = vec![
                Value::Identifier(device_id),
                Value::Ed25519PublicKey(config.device_ed25519_keypair.public),
                Value::Signature(accessory_signature),
            ]
            .encode();

            let mut nonce = vec![0; 4];
            nonce.extend(b"PS-Msg06");

            let mut encrypted_data = Vec::new();
            encrypted_data.extend_from_slice(&encoded_sub_tlv);
            let auth_tag =
                aead.encrypt_in_place_detached(GenericArray::from_slice(&nonce), &[], &mut encrypted_data)?;
            encrypted_data.extend(&auth_tag);

            event_emitter
                .lock()
                .expect("couldn't access event_emitter")
                .emit(&Event::DevicePaired);

            info!("pair setup M6: sending SRP exchange response");

            Ok(vec![
                Value::State(StepNumber::ExchangeRes as u8),
                Value::EncryptedData(encrypted_data),
            ])
        } else {
            Err(tlv::Error::Unknown)
        }
    } else {
        Err(tlv::Error::Unknown)
    }
}

fn verify_client_proof<D: Digest>(
    b_pub: &[u8],
    a_pub: &[u8],
    a_proof: &[u8],
    salt: &[u8],
    key: &[u8],
    group: &SrpGroup,
) -> Result<Vec<u8>, tlv::Error> {
    let mut dhn = D::new();
    dhn.input(&group.n.to_bytes_be());
    let hn = BigUint::from_bytes_be(&dhn.result());

    let mut dhg = D::new();
    dhg.input(&group.g.to_bytes_be());
    let hg = BigUint::from_bytes_be(&dhg.result());

    let hng = hn.bitxor(hg);

    let mut dhi = D::new();
    dhi.input(b"Pair-Setup");
    let hi = dhi.result();

    let mut d = D::new();
    // M = H(H(N) xor H(g), H(I), s, A, B, K)
    d.input(&hng.to_bytes_be());
    d.input(&hi);
    d.input(salt);
    d.input(a_pub);
    d.input(b_pub);
    d.input(key);

    if a_proof == d.result().as_slice() {
        // H(A, M, K)
        let mut d = D::new();
        d.input(a_pub);
        d.input(a_proof);
        d.input(key);
        Ok(d.result().as_slice().to_vec())
    } else {
        Err(tlv::Error::Authentication)
    }
}
