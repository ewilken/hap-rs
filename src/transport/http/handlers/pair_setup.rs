use std::{str, collections::HashMap, ops::BitXor};

use rand::{self, Rng};
use sha2::{Sha512, Digest};
use srp::{
    server::{UserRecord, SrpServer},
    client::{SrpClient, srp_private_key},
    groups::G_3072,
    types::SrpGroup,
};
use num::BigUint;
use ring::{hkdf, hmac, digest};
use chacha20_poly1305_aead;
use crypto::ed25519;
use uuid::Uuid;

use db::database::DatabasePtr;
use config::ConfigPtr;
use transport::{http::handlers::TlvHandler, tlv::{self, Type, Value}};
use protocol::{device::Device, pairing::{Pairing, Permissions}};
use event::{Event, EmitterPtr};

struct Session {
    salt: Vec<u8>,
    verifier: Vec<u8>,
    b: Vec<u8>,
    b_pub: Vec<u8>,
    shared_secret: Option<Vec<u8>>,
}

pub struct PairSetup {
    session: Option<Session>
}

impl PairSetup {
    pub fn new() -> PairSetup {
        PairSetup { session: None }
    }
}

pub enum Step {
    Start,
    Verify { a_pub: Vec<u8>, a_proof: Vec<u8> },
    Exchange { data: Vec<u8> },
}

impl TlvHandler for PairSetup {
    type ParseResult = Step;
    type Result = tlv::Container;

    fn parse(&self, body: Vec<u8>) -> Result<Step, tlv::ErrorContainer> {
        let decoded = tlv::decode(body);
        match decoded.get(&(Type::State as u8)) {
            Some(method) => match method[0] {
                // TODO - put those step numbers into the step enum somehow
                1 => Ok(Step::Start),
                3 => {
                    let a_pub = decoded.get(&(Type::PublicKey as u8)).ok_or(
                        tlv::ErrorContainer::new(4, tlv::Error::Unknown)
                    )?;
                    let a_proof = decoded.get(&(Type::Proof as u8)).ok_or(
                        tlv::ErrorContainer::new(4, tlv::Error::Unknown)
                    )?;
                    Ok(Step::Verify { a_pub: a_pub.clone(), a_proof: a_proof.clone() })
                },
                5 => {
                    let data = decoded.get(&(Type::EncryptedData as u8)).ok_or(
                        tlv::ErrorContainer::new(6, tlv::Error::Unknown)
                    )?;
                    Ok(Step::Exchange { data: data.clone() })
                },
                _ => Err(tlv::ErrorContainer::new(0, tlv::Error::Unknown)),
            },
            None => Err(tlv::ErrorContainer::new(0, tlv::Error::Unknown)),
        }
    }

    fn handle(
        &mut self,
        step: Step,
        config: &ConfigPtr,
        database: &DatabasePtr,
        event_emitter: &EmitterPtr,
    ) -> Result<tlv::Container, tlv::ErrorContainer> {
        match step {
            Step::Start => match handle_start(self, database) {
                Ok(res) => Ok(res),
                Err(err) => Err(tlv::ErrorContainer::new(2, err)),
            },
            Step::Verify { a_pub, a_proof } => match handle_verify(self, a_pub, a_proof) {
                Ok(res) => Ok(res),
                Err(err) => Err(tlv::ErrorContainer::new(4, err)),
            },
            Step::Exchange { data } => match handle_exchange(self, config, database, event_emitter, data) {
                Ok(res) => Ok(res),
                Err(err) => Err(tlv::ErrorContainer::new(6, err)),
            },
        }
    }
}

fn handle_start(
    handler: &mut PairSetup,
    database: &DatabasePtr,
) -> Result<tlv::Container, tlv::Error> {
    println!("/pair-setup - M1: Got SRP Start Request");

    // TODO - Errors for kTLVError_Unavailable, kTLVError_MaxTries and kTLVError_Busy

    let accessory = Device::load(database)?;

    let mut rng = rand::thread_rng();
    let salt = rng.gen_iter::<u8>().take(16).collect::<Vec<u8>>(); // s
    let b = rng.gen_iter::<u8>().take(64).collect::<Vec<u8>>();

    let private_key = srp_private_key::<Sha512>(b"Pair-Setup", accessory.pin.as_bytes(), &salt); // x = H(s | H(I | ":" | P))
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
        salt: salt.clone(),
        verifier: verifier.clone(),
        b: b.clone(),
        b_pub: b_pub.clone(),
        shared_secret: None,
    });

    println!("/pair-setup - M2: Sending SRP Start Response");

    Ok(vec![Value::State(2), Value::PublicKey(b_pub), Value::Salt(salt.clone())])
}

fn handle_verify(
    handler: &mut PairSetup,
    a_pub: Vec<u8>,
    a_proof: Vec<u8>,
) -> Result<tlv::Container, tlv::Error> {
    println!("/pair-setup - M3: Got SRP Verify Request");

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
            &a_pub,
            &a_proof,
            &session.salt,
            &shared_secret.as_slice().to_vec(),
            &G_3072,
        )?;

        println!("/pair-setup - M4: Sending SRP Verify Response");

        Ok(vec![Value::State(4), Value::Proof(b_proof)])
    } else {
        Err(tlv::Error::Unknown)
    }
}

fn handle_exchange(
    handler: &mut PairSetup,
    config: &ConfigPtr,
    database: &DatabasePtr,
    event_emitter: &EmitterPtr,
    data: Vec<u8>,
) -> Result<tlv::Container, tlv::Error> {
    println!("/pair-setup - M5: Got SRP Exchange Request");

    if let Some(ref mut session) = handler.session {
        if let Some(ref mut shared_secret) = session.shared_secret {
            let encrypted_data = Vec::from(&data[..data.len() - 16]);
            let auth_tag = Vec::from(&data[data.len() - 16..]);

            let mut encryption_key = [0; 32];
            let salt = hmac::SigningKey::new(&digest::SHA512, b"Pair-Setup-Encrypt-Salt");
            hkdf::extract_and_expand(&salt, &shared_secret, b"Pair-Setup-Encrypt-Info", &mut encryption_key);

            let mut decrypted_data = Vec::new();
            let mut nonce = vec![0; 4];
            nonce.extend(b"PS-Msg05");
            chacha20_poly1305_aead::decrypt(
                &encryption_key,
                &nonce,
                &[],
                &encrypted_data,
                &auth_tag,
                &mut decrypted_data,
            )?;

            let sub_tlv = tlv::decode(decrypted_data);
            let device_pairing_id = sub_tlv.get(&(Type::Identifier as u8)).ok_or(tlv::Error::Unknown)?;
            let device_ltpk = sub_tlv.get(&(Type::PublicKey as u8)).ok_or(tlv::Error::Unknown)?;
            let device_signature = sub_tlv.get(&(Type::Signature as u8)).ok_or(tlv::Error::Unknown)?;

            let mut device_x = [0; 32];
            let salt = hmac::SigningKey::new(&digest::SHA512, b"Pair-Setup-Controller-Sign-Salt");
            hkdf::extract_and_expand(
                &salt,
                &shared_secret,
                b"Pair-Setup-Controller-Sign-Info",
                &mut device_x,
            );

            let mut device_info: Vec<u8> = Vec::new();
            device_info.extend(&device_x);
            device_info.extend(device_pairing_id);
            device_info.extend(device_ltpk);
            if !ed25519::verify(&device_info, &device_ltpk, &device_signature) {
                return Err(tlv::Error::Authentication);
            }

            let uuid_str = str::from_utf8(device_pairing_id)?;
            let pairing_uuid = Uuid::parse_str(uuid_str)?;
            let mut pairing_ltpk = [0; 32];
            for i in 0..32 {
                pairing_ltpk[i] = device_ltpk[i];
            }

            if let Some(max_peers) = config.borrow().max_peers {
                let d = database.borrow();
                let count = d.count_pairings()?;
                if count + 1 > max_peers {
                    return Err(tlv::Error::MaxPeers);
                }
            }

            let pairing = Pairing::new(pairing_uuid, Permissions::Admin, pairing_ltpk);
            pairing.save(database)?;
            // TODO - kTLVError_MaxPeers

            let mut accessory_x = [0; 32];
            let salt = hmac::SigningKey::new(&digest::SHA512, b"Pair-Setup-Accessory-Sign-Salt");
            hkdf::extract_and_expand(
                &salt,
                &shared_secret,
                b"Pair-Setup-Accessory-Sign-Info",
                &mut accessory_x
            );

            let accessory = Device::load(database)?;
            let mut accessory_info: Vec<u8> = Vec::new();
            accessory_info.extend(&accessory_x);
            accessory_info.extend(accessory.id.as_bytes());
            accessory_info.extend(&accessory.public_key);
            let accessory_signature = ed25519::signature(&accessory_info, &accessory.private_key);

            let mut sub_tlv: HashMap<u8, Vec<u8>> = HashMap::new();
            Value::Identifier(accessory.id).into_map(&mut sub_tlv);
            Value::PublicKey(accessory.public_key.to_vec()).into_map(&mut sub_tlv);
            Value::Signature(accessory_signature.to_vec()).into_map(&mut sub_tlv);
            let encoded_sub_tlv = tlv::encode(sub_tlv);

            let mut encrypted_data = Vec::new();
            let mut nonce = vec![0; 4];
            nonce.extend(b"PS-Msg06");
            let auth_tag = chacha20_poly1305_aead::encrypt(
                &encryption_key,
                &nonce,
                &[],
                &encoded_sub_tlv,
                &mut encrypted_data,
            )?;
            encrypted_data.extend(&auth_tag);

            event_emitter.borrow().emit(Event::DevicePaired);

            println!("/pair-setup - M6: Sending SRP Exchange Response");

            Ok(vec![Value::State(6), Value::EncryptedData(encrypted_data)])
        } else {
            Err(tlv::Error::Unknown)
        }
    } else {
        Err(tlv::Error::Unknown)
    }
}

// TODO - fix the actual srp package to do proper verification
fn verify_client_proof<D: Digest>(
    b_pub: &Vec<u8>,
    a_pub: &Vec<u8>,
    a_proof: &Vec<u8>,
    salt: &Vec<u8>,
    key: &Vec<u8>,
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

    if a_proof.clone() == d.result().as_slice() {
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
