use aead::{generic_array::GenericArray, AeadInPlace, NewAead};
use chacha20poly1305::ChaCha20Poly1305;
use futures::future::{BoxFuture, FutureExt};
use hyper::{body::Buf, Body};
use log::{debug, info};
use num::BigUint;
use rand::{rngs::OsRng, RngCore};
use sha2::{digest::Digest, Sha512};
use signature::{Signature, Signer, Verifier};
use srp::{
    client::{srp_private_key, SrpClient},
    groups::G_3072,
    server::{SrpServer, UserRecord},
    types::SrpGroup,
};
use std::{ops::BitXor, str, time::Duration};
use tokio::time;
use uuid::Uuid;

use crate::{
    event::Event,
    pairing::{Pairing, Permissions},
    pointer,
    tlv::{self, Encodable, Type, Value},
    transport::{hkdf_extract_and_expand, http::handler::TlvHandlerExt},
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
    SrpStartRequest = 1,
    SrpStartResponse = 2,
    SrpVerifyRequest = 3,
    SrpVerifyResponse = 4,
    ExchangeRequest = 5,
    ExchangeResponse = 6,
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
            let aggregated_body = hyper::body::aggregate(body)
                .await
                .map_err(|_| tlv::ErrorContainer::new(StepNumber::Unknown as u8, tlv::Error::Unknown))?;

            debug!("received body: {:?}", aggregated_body.chunk());

            let mut decoded = tlv::decode(aggregated_body.chunk());
            match decoded.get(&(Type::State as u8)) {
                Some(method) => match method[0] {
                    x if x == StepNumber::SrpStartRequest as u8 => Ok(Step::Start),
                    x if x == StepNumber::SrpVerifyRequest as u8 => {
                        let a_pub = decoded
                            .remove(&(Type::PublicKey as u8))
                            .ok_or(tlv::ErrorContainer::new(
                                StepNumber::SrpVerifyResponse as u8,
                                tlv::Error::Unknown,
                            ))?;
                        let a_proof = decoded.remove(&(Type::Proof as u8)).ok_or(tlv::ErrorContainer::new(
                            StepNumber::SrpVerifyResponse as u8,
                            tlv::Error::Unknown,
                        ))?;
                        Ok(Step::Verify { a_pub, a_proof })
                    },
                    x if x == StepNumber::ExchangeRequest as u8 => {
                        let data = decoded
                            .remove(&(Type::EncryptedData as u8))
                            .ok_or(tlv::ErrorContainer::new(
                                StepNumber::ExchangeResponse as u8,
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
                        Err(tlv::ErrorContainer::new(StepNumber::SrpStartResponse as u8, err))
                    },
                },
                Step::Verify { a_pub, a_proof } => match handle_verify(self, &a_pub, &a_proof).await {
                    Ok(res) => {
                        self.unsuccessful_tries = 0;
                        Ok(res)
                    },
                    Err(err) => {
                        self.unsuccessful_tries += 1;
                        Err(tlv::ErrorContainer::new(StepNumber::SrpVerifyResponse as u8, err))
                    },
                },
                Step::Exchange { data } => match handle_exchange(self, config, storage, event_emitter, &data).await {
                    Ok(res) => {
                        self.unsuccessful_tries = 0;
                        Ok(res)
                    },
                    Err(err) => {
                        self.unsuccessful_tries += 1;
                        Err(tlv::ErrorContainer::new(StepNumber::ExchangeResponse as u8, err))
                    },
                },
            }
        }
        .boxed()
    }
}

async fn handle_start(handler: &mut PairSetup, config: pointer::Config) -> Result<tlv::Container, tlv::Error> {
    info!("pair setup M1: received SRP start request");

    // TODO
    // If the accessory is already paired, it must respond with the following TLV items:
    // kTLVType_State <M2>
    // kTLVType_Error <kTLVError_Unavailable>

    if handler.unsuccessful_tries > 100 {
        return Err(tlv::Error::MaxTries);
    }

    // TODO
    // If the accessory is currently performing a PairSetup procedure with a different controller, it must respond with
    // the following TLV items:
    // kTLVType_State <M2>
    // kTLVType_Error <kTLVError_Busy>

    let mut csprng = OsRng {};
    let mut salt = [0; 16]; // s
    let mut b = [0; 64];
    csprng.fill_bytes(&mut salt);
    csprng.fill_bytes(&mut b);

    // TODO - respect pairing flags (specification p. 35 - 7.) for split pair setup

    let private_key = srp_private_key::<Sha512>(b"Pair-Setup", &config.lock().await.pin.to_string().as_bytes(), &salt); // x = H(s | H(I | ":" | P))
    let srp_client = SrpClient::<Sha512>::new(&private_key, &G_3072);
    let verifier = srp_client.get_password_verifier(&private_key); // v = g^x

    let user = UserRecord {
        username: b"Pair-Setup",
        salt: &salt,
        verifier: &verifier,
    };
    let srp_server = SrpServer::<Sha512>::new(&user, b"foo", &b, &G_3072)?;
    let b_pub = srp_server.get_b_pub();

    handler.session = Some(Session {
        salt,
        verifier,
        b,
        b_pub: b_pub.clone(),
        shared_secret: None,
    });

    info!("pair setup M2: sending SRP start response");

    Ok(vec![
        Value::State(StepNumber::SrpStartResponse as u8),
        Value::PublicKey(b_pub),
        Value::Salt(salt),
    ])
}

async fn handle_verify(handler: &mut PairSetup, a_pub: &[u8], a_proof: &[u8]) -> Result<tlv::Container, tlv::Error> {
    info!("pair setup M3: received SRP verify request");

    match handler.session {
        None => Err(tlv::Error::Unknown),
        Some(ref mut session) => {
            let user = UserRecord {
                username: b"Pair-Setup",
                salt: &session.salt,
                verifier: &session.verifier,
            };
            let srp_server = SrpServer::<Sha512>::new(&user, a_pub, &session.b, &G_3072)?;
            let shared_secret = srp_server.get_key();

            session.shared_secret = Some(shared_secret.to_vec());

            let b_proof =
                verify_client_proof::<Sha512>(&session.b_pub, a_pub, a_proof, &session.salt, &shared_secret, &G_3072)?;

            info!("pair setup M4: sending SRP verify response");

            Ok(vec![
                Value::State(StepNumber::SrpVerifyResponse as u8),
                Value::Proof(b_proof),
            ])
        },
    }
}

async fn handle_exchange(
    handler: &mut PairSetup,
    config: pointer::Config,
    storage: pointer::Storage,
    event_emitter: pointer::EventEmitter,
    data: &[u8],
) -> Result<tlv::Container, tlv::Error> {
    info!("pair setup M5: received exchange request");

    match handler.session {
        None => Err(tlv::Error::Unknown),
        Some(ref mut session) => match session.shared_secret {
            None => Err(tlv::Error::Unknown),
            Some(ref shared_secret) => {
                let encrypted_data = Vec::from(&data[..data.len() - 16]);
                let auth_tag = Vec::from(&data[data.len() - 16..]);

                let encryption_key =
                    hkdf_extract_and_expand(b"Pair-Setup-Encrypt-Salt", shared_secret, b"Pair-Setup-Encrypt-Info")?;

                let mut nonce = vec![0; 4];
                nonce.extend(b"PS-Msg05");

                let aead = ChaCha20Poly1305::new(GenericArray::from_slice(&encryption_key));

                let mut decrypted_data = Vec::new();
                decrypted_data.extend_from_slice(&encrypted_data);
                aead.decrypt_in_place_detached(
                    GenericArray::from_slice(&nonce),
                    &[],
                    &mut decrypted_data,
                    GenericArray::from_slice(&auth_tag),
                )?;

                let sub_tlv = tlv::decode(&decrypted_data);
                let device_pairing_id = sub_tlv.get(&(Type::Identifier as u8)).ok_or(tlv::Error::Unknown)?;
                let device_ltpk = ed25519_dalek::PublicKey::from_bytes(
                    sub_tlv.get(&(Type::PublicKey as u8)).ok_or(tlv::Error::Unknown)?,
                )?;
                let device_signature = ed25519_dalek::Signature::from_bytes(
                    sub_tlv.get(&(Type::Signature as u8)).ok_or(tlv::Error::Unknown)?,
                )?;

                let device_x = hkdf_extract_and_expand(
                    b"Pair-Setup-Controller-Sign-Salt",
                    shared_secret,
                    b"Pair-Setup-Controller-Sign-Info",
                )?;

                let mut device_info: Vec<u8> = Vec::new();
                device_info.extend(&device_x);
                device_info.extend(device_pairing_id);
                device_info.extend(device_ltpk.as_bytes());

                if device_ltpk.verify(&device_info, &device_signature).is_err() {
                    return Err(tlv::Error::Authentication);
                }

                let uuid_str = str::from_utf8(device_pairing_id)?;
                let pairing_uuid = Uuid::parse_str(uuid_str)?;
                let mut pairing_ltpk = [0; 32];
                pairing_ltpk[..32].copy_from_slice(&device_ltpk.as_bytes()[..32]);

                if let Some(max_peers) = config.lock().await.max_peers {
                    if storage.lock().await.count_pairings().await? + 1 > max_peers {
                        return Err(tlv::Error::MaxPeers);
                    }
                }

                let pairing = Pairing::new(pairing_uuid, Permissions::Admin, device_ltpk.to_bytes());
                storage.lock().await.save_pairing(&pairing).await?;

                debug!("pairing: {:?}", &pairing);

                let accessory_x = hkdf_extract_and_expand(
                    b"Pair-Setup-Accessory-Sign-Salt",
                    shared_secret,
                    b"Pair-Setup-Accessory-Sign-Info",
                )?;

                let config = config.lock().await;
                let device_id = config.device_id.to_hex_string();

                let mut accessory_info: Vec<u8> = Vec::new();
                accessory_info.extend(&accessory_x);
                accessory_info.extend(device_id.as_bytes());
                accessory_info.extend(config.device_ed25519_keypair.public.as_bytes());
                let accessory_signature = config.device_ed25519_keypair.sign(&accessory_info);

                let encoded_sub_tlv = vec![
                    Value::Identifier(device_id),
                    Value::PublicKey(config.device_ed25519_keypair.public.as_bytes().to_vec()),
                    Value::Signature(accessory_signature.to_bytes().to_vec()),
                ]
                .encode();

                drop(config);

                let mut nonce = vec![0; 4];
                nonce.extend(b"PS-Msg06");

                let mut encrypted_data = Vec::new();
                encrypted_data.extend_from_slice(&encoded_sub_tlv);
                let auth_tag =
                    aead.encrypt_in_place_detached(GenericArray::from_slice(&nonce), &[], &mut encrypted_data)?;
                encrypted_data.extend(&auth_tag);

                let id = pairing.id;
                tokio::spawn(async move {
                    // not deferring this might make the iOS controller drop the connection if the Bonjour txt records
                    // change before the end of PairSetup
                    time::sleep(Duration::from_secs(5)).await;

                    event_emitter.lock().await.emit(&Event::ControllerPaired { id }).await;
                });

                info!("pair setup M6: sending exchange response");

                Ok(vec![
                    Value::State(StepNumber::ExchangeResponse as u8),
                    Value::EncryptedData(encrypted_data),
                ])
            },
        },
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
    dhn.update(&group.n.to_bytes_be());
    let hn = BigUint::from_bytes_be(&dhn.finalize());

    let mut dhg = D::new();
    dhg.update(&group.g.to_bytes_be());
    let hg = BigUint::from_bytes_be(&dhg.finalize());

    let hng = hn.bitxor(hg);

    let mut dhi = D::new();
    dhi.update(b"Pair-Setup");
    let hi = dhi.finalize();

    let mut d = D::new();
    // M = H(H(N) xor H(g), H(I), s, A, B, K)
    d.update(&hng.to_bytes_be());
    d.update(&hi);
    d.update(salt);
    d.update(a_pub);
    d.update(b_pub);
    d.update(key);

    if a_proof == d.finalize().as_slice() {
        // H(A, M, K)
        let mut d = D::new();
        d.update(a_pub);
        d.update(a_proof);
        d.update(key);
        Ok(d.finalize().as_slice().to_vec())
    } else {
        Err(tlv::Error::Authentication)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_client_proof() {
        let b_pub = [
            66, 254, 195, 54, 248, 36, 231, 56, 186, 118, 54, 42, 5, 66, 113, 73, 150, 54, 54, 156, 16, 76, 252, 50,
            206, 135, 204, 173, 159, 174, 68, 125, 23, 251, 75, 215, 136, 102, 165, 73, 107, 249, 205, 158, 32, 192,
            157, 134, 17, 70, 51, 225, 74, 3, 195, 74, 124, 137, 26, 48, 245, 253, 212, 197, 29, 141, 12, 5, 36, 191,
            77, 203, 137, 72, 27, 38, 231, 136, 43, 60, 134, 137, 192, 134, 218, 13, 90, 232, 166, 245, 168, 29, 34,
            91, 121, 185, 82, 144, 146, 215, 6, 120, 204, 126, 217, 146, 135, 243, 55, 1, 99, 159, 192, 53, 80, 103,
            19, 87, 253, 193, 104, 98, 228, 167, 138, 214, 84, 135, 232, 225, 185, 158, 114, 48, 228, 110, 137, 109,
            32, 21, 227, 177, 18, 241, 240, 240, 125, 115, 135, 46, 249, 172, 206, 170, 41, 76, 169, 163, 62, 77, 94,
            51, 116, 69, 188, 218, 171, 90, 171, 128, 163, 84, 233, 52, 134, 77, 70, 130, 207, 84, 55, 218, 155, 199,
            67, 214, 10, 25, 142, 134, 34, 65, 43, 52, 113, 92, 234, 213, 101, 32, 12, 34, 87, 31, 82, 132, 175, 144,
            116, 36, 47, 119, 221, 46, 103, 44, 80, 74, 201, 196, 249, 230, 127, 123, 80, 118, 216, 163, 249, 186, 218,
            184, 181, 144, 72, 187, 101, 164, 150, 209, 45, 247, 70, 184, 92, 155, 146, 175, 162, 54, 95, 155, 174,
            116, 38, 190, 166, 137, 133, 51, 253, 21, 169, 129, 167, 61, 102, 49, 115, 24, 245, 130, 171, 6, 196, 177,
            44, 49, 149, 39, 239, 142, 253, 49, 123, 226, 140, 40, 43, 193, 131, 20, 30, 17, 57, 252, 1, 68, 55, 1,
            227, 226, 24, 73, 204, 237, 230, 2, 81, 203, 54, 204, 157, 174, 22, 134, 126, 132, 106, 66, 115, 27, 189,
            157, 131, 173, 205, 36, 140, 160, 183, 171, 62, 97, 7, 73, 93, 114, 199, 50, 192, 22, 56, 132, 172, 61,
            208, 84, 102, 85, 60, 216, 170, 249, 29, 66, 158, 48, 221, 78, 166, 50, 20, 203, 99, 206, 6, 112, 163, 194,
            43, 165, 176, 193, 62, 157, 184, 57, 235, 221, 175, 46, 7, 152,
        ];
        let a_pub = [
            21, 187, 129, 247, 180, 115, 7, 3, 46, 219, 190, 0, 89, 8, 126, 225, 37, 159, 98, 137, 226, 202, 13, 228,
            31, 55, 17, 244, 180, 120, 159, 97, 141, 126, 131, 219, 227, 64, 81, 24, 99, 206, 12, 218, 27, 95, 43, 216,
            151, 228, 46, 89, 133, 153, 100, 55, 134, 206, 65, 165, 244, 228, 121, 88, 78, 54, 19, 220, 48, 86, 150,
            92, 21, 168, 219, 226, 214, 254, 33, 26, 135, 134, 247, 49, 58, 114, 245, 76, 67, 182, 223, 191, 93, 1,
            131, 58, 76, 62, 49, 123, 11, 10, 164, 221, 249, 114, 77, 154, 179, 95, 207, 127, 9, 154, 30, 91, 99, 67,
            160, 94, 33, 239, 178, 32, 173, 245, 25, 91, 153, 189, 34, 216, 16, 100, 160, 145, 150, 208, 85, 135, 237,
            13, 225, 234, 144, 137, 224, 217, 187, 77, 10, 51, 240, 45, 72, 9, 184, 164, 195, 153, 55, 66, 98, 110,
            208, 204, 45, 204, 14, 46, 195, 119, 25, 83, 120, 223, 17, 167, 79, 75, 68, 182, 105, 97, 176, 15, 153, 95,
            170, 16, 33, 172, 134, 114, 73, 69, 5, 49, 78, 133, 250, 44, 136, 75, 179, 33, 213, 48, 77, 236, 16, 148,
            221, 74, 185, 209, 83, 94, 167, 180, 101, 170, 91, 129, 143, 228, 180, 251, 185, 210, 16, 74, 26, 248, 65,
            208, 73, 182, 161, 120, 59, 223, 98, 166, 58, 94, 69, 250, 93, 75, 29, 36, 41, 211, 242, 115, 253, 65, 28,
            204, 117, 167, 194, 113, 98, 180, 54, 96, 170, 81, 125, 134, 19, 213, 2, 213, 178, 163, 108, 74, 215, 177,
            52, 217, 75, 225, 144, 220, 3, 164, 87, 156, 233, 1, 99, 218, 251, 89, 45, 165, 227, 98, 232, 143, 165,
            141, 246, 196, 230, 209, 136, 35, 75, 218, 63, 15, 94, 150, 79, 113, 245, 247, 28, 139, 115, 136, 232, 175,
            49, 228, 11, 70, 234, 131, 126, 167, 31, 234, 202, 34, 27, 240, 195, 201, 1, 29, 98, 229, 254, 146, 87,
            223, 113, 5, 218, 217, 250, 58, 115, 53, 9, 162, 175, 42, 176, 10, 195, 48, 155, 56, 58, 246, 219, 98, 166,
            41, 34, 216, 225, 29, 28, 1, 18, 89,
        ];
        let a_proof = [
            33, 202, 204, 21, 29, 11, 142, 163, 254, 113, 245, 137, 104, 89, 101, 247, 182, 215, 41, 42, 213, 171, 173,
            142, 172, 183, 214, 187, 204, 48, 253, 153, 224, 246, 18, 246, 72, 5, 95, 165, 27, 245, 255, 22, 229, 250,
            129, 33, 8, 40, 1, 194, 131, 19, 51, 75, 37, 179, 34, 60, 222, 13, 182, 81,
        ];
        let salt = [67, 81, 86, 223, 10, 171, 140, 180, 0, 188, 111, 77, 228, 110, 173, 185];
        let shared_secret = [
            39, 162, 132, 15, 245, 98, 72, 107, 190, 101, 117, 162, 228, 189, 241, 14, 132, 216, 104, 7, 65, 72, 90,
            154, 86, 129, 171, 235, 197, 55, 174, 216, 183, 170, 12, 101, 219, 128, 62, 155, 113, 212, 250, 40, 137,
            178, 199, 215, 68, 139, 218, 112, 205, 68, 52, 66, 95, 11, 116, 251, 143, 93, 206, 89,
        ];

        let b_proof = verify_client_proof::<Sha512>(&b_pub, &a_pub, &a_proof, &salt, &shared_secret, &G_3072).unwrap();

        assert_eq!(b_proof, vec![
            53, 222, 231, 209, 7, 123, 202, 208, 135, 119, 183, 90, 79, 154, 55, 155, 63, 56, 215, 210, 4, 20, 229,
            119, 234, 168, 107, 137, 48, 172, 180, 244, 184, 142, 170, 120, 188, 106, 94, 135, 122, 4, 211, 21, 190,
            26, 121, 180, 13, 192, 173, 246, 172, 223, 161, 192, 52, 251, 187, 66, 52, 170, 18, 85
        ]);
    }
}
