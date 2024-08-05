use std::{cell, collections::HashMap, io, str};

use base64::prelude::*;
use byteorder::{LittleEndian, WriteBytesExt};
use log::error;
use srp::types::SrpAuthError;
use thiserror::Error;

use crate::{error, pairing::Permissions};

#[derive(Clone, Default, Debug)]
pub struct Tlv8(pub Vec<u8>);

impl serde::Serialize for Tlv8 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let result = BASE64_STANDARD.encode(&self.0);

        serializer.serialize_str(&result)
    }
}

impl<'de> serde::Deserialize<'de> for Tlv8 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum Mixed {
            Plain(String),
            Vec(Vec<u8>),
        }

        match Mixed::deserialize(deserializer)? {
            Mixed::Plain(plain) => {
                let result = BASE64_STANDARD.decode(plain).map_err(Error::custom)?;
                Ok(Self(result))
            },
            Mixed::Vec(bytes) => Ok(Self(bytes)),
        }
    }
}

/// Encodes a `Vec<(u8, Vec<u8>)>` in the format `(<Type>, <Value>)` to a `Vec<u8>` of concatenated TLVs.
pub fn encode(tlvs: Vec<(u8, Vec<u8>)>) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    for (t, v) in tlvs {
        let length = v.len();
        if length <= 255 {
            vec.push(t);
            vec.push(length as u8);
            for byte in v {
                vec.push(byte);
            }
        } else {
            let mut l = length;
            let mut p = 0;
            while l > 255 {
                vec.push(t);
                vec.push(255);
                for byte in &v[p..(p + 255)] {
                    vec.push(*byte);
                }
                l -= 255;
                p += 255;
            }
            if l > 0 {
                vec.push(t);
                vec.push(l as u8);
                for byte in &v[p..(p + l)] {
                    vec.push(*byte);
                }
            }
        }
    }
    vec
}

/// Decodes a slice of concatenated TLVs to a `HashMap<u8, Vec<u8>>` in the format `<Type, Value>`.
pub fn decode(tlv: &[u8]) -> HashMap<u8, Vec<u8>> {
    let mut hm = HashMap::new();
    let mut buf: Vec<u8> = Vec::new();
    let mut p = 0;
    let mut pt = 0;
    while p < tlv.len() {
        let t = tlv[p];
        let l = tlv[p + 1];
        if l < 255 {
            if t != pt && !buf.is_empty() {
                hm.insert(t, buf.clone());
                buf.clear();
            }
            buf.extend_from_slice(&tlv[p + 2..p + 2 + l as usize]);
            hm.insert(t, buf.clone());
            buf.clear();
        } else {
            buf.extend_from_slice(&tlv[p + 2..p + 2 + l as usize]);
        }
        pt = t;
        p = p + 2 + l as usize;
    }
    if !buf.is_empty() {
        hm.insert(pt, buf.clone());
        buf.clear();
    }
    hm
}

/// `Encodable` is implemented by types that can be encoded to a to a `Vec<u8>` of concatenated
/// TLVs.
pub trait Encodable {
    fn encode(self) -> Vec<u8>;
}

/// `Type` represents the TLV types defined by the protocol.
#[derive(Debug, Copy, Clone)]
pub enum Type {
    Method = 0x00,
    Identifier = 0x01,
    Salt = 0x02,
    PublicKey = 0x03,
    Proof = 0x04,
    EncryptedData = 0x05,
    State = 0x06,
    Error = 0x07,
    RetryDelay = 0x08,
    Certificate = 0x09,
    Signature = 0x0A,
    Permissions = 0x0B,
    FragmentData = 0x0C,
    FragmentLast = 0x0D,
    /// Pairing Type Flags (32 bit unsigned integer).
    Flags = 0x13,
    Separator = 0xFF,
}

/// The variants of `Value` can hold the corresponding values to the types provided by `Type`.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Value {
    Method(Method),
    Identifier(String),
    Salt([u8; 16]),
    PublicKey(Vec<u8>),
    Proof(Vec<u8>),
    EncryptedData(Vec<u8>),
    State(u8),
    Error(Error),
    RetryDelay(usize),
    Certificate(Vec<u8>),
    Signature(Vec<u8>),
    Permissions(Permissions),
    FragmentData(Vec<u8>),
    FragmentLast(Vec<u8>),
    Flags(u32),
    Separator,
}

impl Value {
    /// Converts a variant of `Value` to a `(u8, Vec<u8>)` tuple in the format `(Type, Value)`.
    pub fn as_tlv(self) -> (u8, Vec<u8>) {
        match self {
            Value::Method(method) => (Type::Method as u8, vec![method as u8]),
            Value::Identifier(identifier) => (Type::Identifier as u8, identifier.into_bytes()),
            Value::Salt(salt) => (Type::Salt as u8, salt.to_vec()),
            Value::PublicKey(public_key) => (Type::PublicKey as u8, public_key),
            Value::Proof(proof) => (Type::Proof as u8, proof),
            Value::EncryptedData(data) => (Type::EncryptedData as u8, data),
            Value::State(state) => (Type::State as u8, vec![state]),
            Value::Error(error) => (Type::Error as u8, vec![error as u8]),
            Value::RetryDelay(delay) => {
                let val = delay as u16;
                let mut vec: Vec<u8> = Vec::new();
                vec.write_u16::<LittleEndian>(val).unwrap();
                (Type::RetryDelay as u8, vec)
            },
            Value::Certificate(certificate) => (Type::Certificate as u8, certificate),
            Value::Signature(signature) => (Type::Signature as u8, signature),
            Value::Permissions(permissions) => (Type::Permissions as u8, vec![permissions.as_byte()]),
            Value::FragmentData(fragment_data) => (Type::FragmentData as u8, fragment_data),
            Value::FragmentLast(fragment_last) => (Type::FragmentLast as u8, fragment_last),
            Value::Flags(flags) => {
                let mut vec: Vec<u8> = Vec::new();
                vec.write_u32::<LittleEndian>(flags).unwrap();
                (Type::Flags as u8, vec)
            },
            Value::Separator => (Type::Separator as u8, vec![0x00]),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Method {
    PairSetup = 1,
    PairVerify = 2,
    AddPairing = 3,
    RemovePairing = 4,
    ListPairings = 5,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Error)]
pub enum Error {
    #[error("Generic error to handle unexpected errors.")]
    Unknown = 0x01,
    #[error("Setup code or signature verification failed.")]
    Authentication = 0x02,
    #[error("Client must look at the retry delay TLV item and wait that many seconds before retrying.")]
    Backoff = 0x03,
    #[error("Server cannot accept any more pairings.")]
    MaxPeers = 0x04,
    #[error("Server reached its maximum number of authentication attempts.")]
    MaxTries = 0x05,
    #[error("Server pairing method is unavailable.")]
    Unavailable = 0x06,
    #[error("Server is busy and cannot accept a pairing request at this time.")]
    Busy = 0x07,
}

impl From<error::Error> for Error {
    fn from(err: error::Error) -> Self {
        error!("{:?}", err);
        Error::Unknown
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        error!("{:?}", err);
        Error::Unknown
    }
}

impl From<cell::BorrowError> for Error {
    fn from(err: cell::BorrowError) -> Error {
        error!("{:?}", err);
        Error::Unknown
    }
}

impl From<cell::BorrowMutError> for Error {
    fn from(err: cell::BorrowMutError) -> Error {
        error!("{:?}", err);
        Error::Unknown
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        error!("{:?}", err);
        Error::Unknown
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        error!("{:?}", err);
        Error::Unknown
    }
}

impl From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        error!("{:?}", err);
        Error::Unknown
    }
}

impl From<SrpAuthError> for Error {
    fn from(err: SrpAuthError) -> Self {
        error!("{:?}", err);
        Error::Authentication
    }
}

impl From<aead::Error> for Error {
    fn from(err: aead::Error) -> Self {
        error!("{:?}", err);
        Error::Authentication
    }
}

impl From<ed25519_dalek::SignatureError> for Error {
    fn from(err: ed25519_dalek::SignatureError) -> Self {
        error!("{:?}", err);
        Error::Authentication
    }
}

pub type Container = Vec<Value>;

impl Encodable for Container {
    fn encode(self) -> Vec<u8> {
        encode(self.into_iter().map(|v| v.as_tlv()).collect::<Vec<_>>())
    }
}

pub struct ErrorContainer {
    step: u8,
    error: Error,
}

impl ErrorContainer {
    pub fn new(step: u8, error: Error) -> ErrorContainer {
        ErrorContainer { step, error }
    }
}

impl Encodable for ErrorContainer {
    fn encode(self) -> Vec<u8> {
        vec![Value::State(self.step), Value::Error(self.error)].encode()
    }
}
