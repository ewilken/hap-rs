use std::collections::HashMap;
use byteorder::{LittleEndian, WriteBytesExt};

pub struct Tlv {
    pub tag: u8,
    pub length: u8,
    pub value: Vec<u8>,
}

impl Tlv {
    pub fn new(tag: u8, length: u8, value: Vec<u8>) -> Tlv {
        Tlv {tag, length, value}
    }
}

pub enum Type {
    Method(MethodKind),
    Identifier(String),
    Salt(Vec<u8>),
    PublicKey(Vec<u8>),
    Proof(Vec<u8>),
    EncryptedData(Vec<u8>),
    State(u8),
    Error(ErrorKind),
    RetryDelay(usize),
    Certificate(Vec<u8>),
    Signature(Vec<u8>),
    Permissions(PermissionsKind),
    FragmentData(Vec<u8>),
    FragmentLast(Vec<u8>),
    Separator,
}

impl Type {
    pub fn as_u8(&self) -> u8 {
        match self {
            &Type::Method(_) => 0x00,
            &Type::Identifier(_) => 0x01,
            &Type::Salt(_) => 0x02,
            &Type::PublicKey(_) => 0x03,
            &Type::Proof(_) => 0x04,
            &Type::EncryptedData(_) => 0x05,
            &Type::State(_) => 0x06,
            &Type::Error(_) => 0x07,
            &Type::RetryDelay(_) => 0x08,
            &Type::Certificate(_) => 0x09,
            &Type::Signature(_) => 0x0A,
            &Type::Permissions(_) => 0x0B,
            &Type::FragmentData(_) => 0x0C,
            &Type::FragmentLast(_) => 0x0D,
            &Type::Separator => 0xFF,
        }
    }
    pub fn as_type_value(self) -> (u8, Vec<u8>) {
        match self {
            Type::Method(method_kind) => (0x00, vec![method_kind.as_u8()]),
            Type::Identifier(identifier) => (0x01, identifier.into_bytes()),
            Type::Salt(salt) => (0x02, salt),
            Type::PublicKey(public_key) => (0x03, public_key),
            Type::Proof(proof) => (0x04, proof),
            Type::EncryptedData(data) => (0x05, data),
            Type::State(state) => (0x06, vec![state]),
            Type::Error(error_kind) => (0x07, vec![error_kind.as_u8()]),
            Type::RetryDelay(delay) => {
                let val = delay as u16;
                let mut vec: Vec<u8> = Vec::new();
                vec.write_u16::<LittleEndian>(val).unwrap();
                (0x08, vec)
            },
            Type::Certificate(certificate) => (0x09, certificate),
            Type::Signature(signature) => (0x0A, signature),
            Type::Permissions(permissions_kind) => (0x0B, vec![permissions_kind.as_u8()]),
            Type::FragmentData(fragment_data) => (0x0C, fragment_data),
            Type::FragmentLast(fragment_last) => (0x0D, fragment_last),
            Type::Separator => (0xFF, vec![0x00]),
        }
    }
}

pub enum MethodKind {
    PairSetup,
    PairVerify,
    AddPairing,
    RemovePairing,
    ListPairings,
}

impl MethodKind {
    pub fn as_u8(&self) -> u8 {
        match self {
            &MethodKind::PairSetup => 1,
            &MethodKind::PairVerify => 2,
            &MethodKind::AddPairing => 3,
            &MethodKind::RemovePairing => 4,
            &MethodKind::ListPairings => 5,
        }
    }
}

pub enum PermissionsKind {
    User,
    Admin,
}

impl PermissionsKind {
    pub fn as_u8(&self) -> u8 {
        match self {
            &PermissionsKind::User => 0x00,
            &PermissionsKind::Admin => 0x01,
        }
    }
}

pub enum ErrorKind {
    Unknown,
    Authentication,
    Backoff,
    MaxPeers,
    MaxTries,
    Unavailable,
    Busy,
}

impl ErrorKind {
    pub fn as_u8(&self) -> u8 {
        match self {
            &ErrorKind::Unknown => 0x01,
            &ErrorKind::Authentication => 0x02,
            &ErrorKind::Backoff => 0x03,
            &ErrorKind::MaxPeers => 0x04,
            &ErrorKind::MaxTries => 0x05,
            &ErrorKind::Unavailable => 0x06,
            &ErrorKind::Busy => 0x07,
        }
    }
}

pub fn encode(hm: HashMap<u8, Vec<u8>>) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    for (k, v) in hm.iter() {
        vec.push(k.to_owned());
        let length = v.len() as u8;
        vec.push(length);
        for byte in v {
            vec.push(byte.to_owned());
        }
    };
    vec
}
