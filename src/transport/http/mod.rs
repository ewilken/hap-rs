use hyper::{server::Response, header::{self, ContentLength}, StatusCode};

pub mod server;
pub mod handlers;
pub mod encrypted_stream;

pub enum Status {
    Success = 0,
    InsufficientPrivileges = -70401,
    ServiceCommunicationFailure = -70402,
    ResourceBusy = -70403,
    ReadOnlyCharacteristic = -70404,
    WriteOnlyCharacteristic = -70405,
    NotificationNotSupported = -70406,
    OutOfResource = -70407,
    OperationTimedOut = -70408,
    ResourceDoesNotExist = -70409,
    InvalidValueInRequest = -70410,
}

enum ContentType {
    PairingTLV8,
    HapJson,
}

impl ContentType {
    pub fn for_hyper(&self) -> header::ContentType {
        match self {
            &ContentType::PairingTLV8 => header::ContentType("application/pairing+tlv8".parse().unwrap()),
            &ContentType::HapJson => header::ContentType("application/hap+json".parse().unwrap()),
        }
    }
}

pub fn tlv_response(body: Vec<u8>, status: StatusCode) -> Response {
    response(body, status, ContentType::PairingTLV8)
}

pub fn json_response(body: Vec<u8>, status: StatusCode) -> Response {
    response(body, status, ContentType::HapJson)
}

pub fn status_response(status: StatusCode) -> Response {
    Response::new().with_status(status)
}

fn response(body: Vec<u8>, status: StatusCode, content_type: ContentType) -> Response {
    Response::new()
        .with_status(status)
        .with_header(ContentLength(body.len() as u64))
        .with_header(content_type.for_hyper())
        .with_body(body)
}
