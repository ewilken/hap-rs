use std::collections::HashMap;
use iron::status;
use iron::prelude::Response;

use transport::tlv;

pub mod server;
pub mod router;
pub mod handlers;

pub enum Status {
    Success,
    InsufficientPrivileges,
    ServiceCommunicationFailure,
    ResourceBusy,
    ReadOnlyCharacteristic,
    WriteOnlyCharacteristic,
    NotificationNotSupported,
    OutOfResource,
    OperationTimedOut,
    ResourceDoesNotExist,
    InvalidValueInRequest,
}

impl Status {
    pub fn as_i32(&self) -> i32 {
        match self {
            &Status::Success => 0,
            &Status::InsufficientPrivileges => -70401,
            &Status::ServiceCommunicationFailure => -70402,
            &Status::ResourceBusy => -70403,
            &Status::ReadOnlyCharacteristic => -70404,
            &Status::WriteOnlyCharacteristic => -70405,
            &Status::NotificationNotSupported => -70406,
            &Status::OutOfResource => -70407,
            &Status::OperationTimedOut => -70408,
            &Status::ResourceDoesNotExist => -70409,
            &Status::InvalidValueInRequest => -70410,
        }
    }
}

pub enum ContentType {
    PairingTLV8,
    HapJson,
}

impl ContentType {
    pub fn as_vec(&self) -> Vec<u8> {
        match self {
            &ContentType::PairingTLV8 => b"application/pairing+tlv8".to_vec(),
            &ContentType::HapJson => b"application/hap+json".to_vec(),
        }
    }
}

pub fn response(answer: HashMap<u8, Vec<u8>>, content_type: ContentType) -> Response {
    let body = tlv::encode(answer);
    let mut response = Response::with((status::Ok, body));
    response.headers.set_raw("Content-Type", vec![content_type.as_vec()]);
    response
}
