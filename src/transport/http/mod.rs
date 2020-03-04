use hyper::{
    header::{CONTENT_LENGTH, CONTENT_TYPE},
    Body,
    Response,
    StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::{
    characteristic::{Format, Perm, Unit},
    Error,
    HapType,
    Result,
};

mod handler;

pub(crate) mod server;

#[allow(dead_code)]
#[derive(Debug)]
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

#[derive(Debug)]
enum ContentType {
    PairingTLV8,
    HapJson,
}

impl ContentType {
    pub fn to_string(self) -> String {
        match self {
            ContentType::PairingTLV8 => "application/pairing+tlv8".into(),
            ContentType::HapJson => "application/hap+json".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CharacteristicResponseBody<T> {
    characteristics: Vec<T>,
}

#[derive(Debug, Default, Serialize)]
pub struct ReadResponseObject {
    pub iid: u64,
    pub aid: u64,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub hap_type: Option<HapType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perms: Option<Vec<Perm>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ev: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<Unit>,
    #[serde(rename = "maxValue", skip_serializing_if = "Option::is_none")]
    pub max_value: Option<serde_json::Value>,
    #[serde(rename = "minValue", skip_serializing_if = "Option::is_none")]
    pub min_value: Option<serde_json::Value>,
    #[serde(rename = "minStep", skip_serializing_if = "Option::is_none")]
    pub step_value: Option<serde_json::Value>,
    #[serde(rename = "maxLen", skip_serializing_if = "Option::is_none")]
    pub max_len: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct WriteObject {
    pub iid: u64,
    pub aid: u64,
    pub ev: Option<bool>,
    pub value: Option<serde_json::Value>,
    #[serde(rename = "authData")]
    pub auth_data: Option<String>,
    pub remote: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct WriteResponseObject {
    pub iid: u64,
    pub aid: u64,
    pub status: i32,
}

#[derive(Debug, Serialize)]
pub struct EventObject {
    pub iid: u64,
    pub aid: u64,
    pub value: serde_json::Value,
}

pub fn tlv_response(body: Vec<u8>, status: StatusCode) -> Result<Response<Body>> {
    response(body, status, ContentType::PairingTLV8)
}

pub fn json_response(body: Vec<u8>, status: StatusCode) -> Result<Response<Body>> {
    response(body, status, ContentType::HapJson)
}

pub fn status_response(status: StatusCode) -> Result<Response<Body>> {
    Response::builder()
        .status(status)
        .body(Body::empty())
        .map_err(Error::from)
}

pub fn event_response(event_objects: Vec<EventObject>) -> Result<Vec<u8>> {
    let body = serde_json::to_string(&CharacteristicResponseBody {
        characteristics: event_objects,
    })?;
    let response = format!(
        "EVENT/1.0 200 OK\nContent-Type: {}\nContent-Length: {}\n\n{}",
        ContentType::HapJson.to_string(),
        body.len(),
        body,
    );
    Ok(response.as_bytes().to_vec())
}

fn response(body: Vec<u8>, status: StatusCode, content_type: ContentType) -> Result<Response<Body>> {
    Response::builder()
        .status(status)
        .header(CONTENT_TYPE, content_type.to_string())
        .header(CONTENT_LENGTH, body.len() as u64)
        .body(body.into())
        .map_err(Error::from)
}
