use std::{fmt, io, str, error, collections::HashMap};

use byteorder::{LittleEndian, WriteBytesExt};
use srp::types::SrpAuthError;
use chacha20_poly1305_aead::DecryptError;
use uuid;

use protocol::pairing::Permissions;

pub fn encode(hm: HashMap<u8, Vec<u8>>) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    for (k, v) in hm.iter() {
        let length = v.len();
        if length <= 255 {
            vec.push(k.clone());
            vec.push(length as u8);
            for byte in v {
                vec.push(byte.clone());
            }
        } else {
            let mut l = length;
            let mut p = 0;
            while l > 255 {
                vec.push(k.clone());
                vec.push(255);
                for byte in &v[p..(p + 255)] {
                    vec.push(byte.clone());
                }
                l -= 255;
                p += 255;
            }
            if l > 0 {
                vec.push(k.clone());
                vec.push(l as u8);
                for byte in &v[p..(p + l)] {
                    vec.push(byte.clone());
                }
            }
        }
    };
    vec
}

pub fn decode(tlv: Vec<u8>) -> HashMap<u8, Vec<u8>> {
    let mut hm = HashMap::new();
    let mut buf: Vec<u8> = Vec::new();
    let mut p = 0;
    let mut pt = 0;
    while p < tlv.len() {
        let t = tlv[p];
        let l = tlv[p + 1];
        if l < 255 {
            if t != pt && buf.len() > 0 {
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
    if buf.len() > 0 {
        hm.insert(pt, buf.clone());
        buf.clear();
    }
    hm
}

pub trait Encodable {
    fn encode(self) -> Vec<u8>;
}

#[derive(Copy, Clone)]
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
    Separator = 0xFF,
}

pub enum Value {
    Method(Method),
    Identifier(String),
    Salt(Vec<u8>),
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
    Separator,
}

impl Value {
    pub fn as_tlv(self) -> (u8, Vec<u8>) {
        match self {
            Value::Method(method_kind) => (Type::Method as u8, vec![method_kind as u8]),
            Value::Identifier(identifier) => (Type::Identifier as u8, identifier.into_bytes()),
            Value::Salt(salt) => (Type::Salt as u8, salt),
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
            Value::Permissions(permissions) => (Type::Permissions as u8, vec![permissions.as_u8()]),
            Value::FragmentData(fragment_data) => (Type::FragmentData as u8, fragment_data),
            Value::FragmentLast(fragment_last) => (Type::FragmentLast as u8, fragment_last),
            Value::Separator => (Type::Separator as u8, vec![0x00]),
        }
    }

    pub fn into_map(self, map: &mut HashMap<u8, Vec<u8>>) {
        let (t, v) = self.as_tlv();
        map.insert(t, v);
    }
}

#[derive(Copy, Clone)]
pub enum Method {
    PairSetup = 1,
    PairVerify = 2,
    AddPairing = 3,
    RemovePairing = 4,
    ListPairings = 5,
}

#[derive(Copy, Clone, Debug)]
pub enum Error {
    Unknown = 0x01,
    Authentication = 0x02,
    Backoff = 0x03,
    MaxPeers = 0x04,
    MaxTries = 0x05,
    Unavailable = 0x06,
    Busy = 0x07,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Unknown => f.write_str("Unknown"),
            Error::Authentication => f.write_str("Authentication"),
            Error::Backoff => f.write_str("Backoff"),
            Error::MaxPeers => f.write_str("MaxPeers"),
            Error::MaxTries => f.write_str("MaxTries"),
            Error::Unavailable => f.write_str("Unavailable"),
            Error::Busy => f.write_str("Busy"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Unknown => "Unknown error",
            Error::Authentication => "Setup code or signature verification failed",
            Error::Backoff => "Client must look at the retry delay TLV item and wait that many seconds before retrying",
            Error::MaxPeers => "Server cannot accept any more pairings",
            Error::MaxTries => "Server reached its maximum number of authentication attempts",
            Error::Unavailable => "Server pairing method is unavailable",
            Error::Busy => "Server is busy and cannot accept a pairing request at this time",
        }
    }
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Error::Unknown
    }
}

impl From<str::Utf8Error> for Error {
    fn from(_: str::Utf8Error) -> Self {
        Error::Unknown
    }
}

impl From<uuid::ParseError> for Error {
    fn from(_: uuid::ParseError) -> Self {
        Error::Unknown
    }
}

impl From<SrpAuthError> for Error {
    fn from(_: SrpAuthError) -> Self {
        Error::Authentication
    }
}

impl From<DecryptError> for Error {
    fn from(_: DecryptError) -> Self {
        Error::Authentication
    }
}

pub type Container = Vec<Value>;

impl Encodable for Container {
    fn encode(self) -> Vec<u8> {
        let mut map = HashMap::new();
        for value in self {
            value.into_map(&mut map);
        }
        encode(map)
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
        let mut map = HashMap::new();
        Value::State(self.step).into_map(&mut map);
        Value::Error(self.error).into_map(&mut map);
        encode(map)
    }
}
