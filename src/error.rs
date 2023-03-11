use std::{io, num, str, sync::mpsc};
use thiserror::Error;

use crate::characteristic::Format;

/// HAP error representation.
#[derive(Debug, Error)]
pub enum Error {
    // custom errors
    #[error("The PIN is too easy. The following PINs are not allowed: []")]
    PinTooEasy,
    #[error("The PIN contains invalid digits. You may only use numbers from 0 to 9.")]
    InvalidPin,
    #[error(
        "Invalid pairing permission Byte: {0}. Only `Permissions::User = 0x00` and `Permissions::Admin = 0x01` are allowed."
    )]
    InvalidPairingPermission(u8),
    #[error("The value is below the `min_value` of the characteristic.")]
    ValueBelowMinValue,
    #[error("The value is above the `max_value` of the characteristic.")]
    ValueAboveMaxValue,
    #[error("The selected accessory is not present on the server.")]
    AccessoryNotFound,
    #[error("The provided accessory was already added to the server.")]
    DuplicateAccessory,
    #[error(
        "The provided value has an invalid data type for the characteristic. The characteristic's format is {0:?}."
    )]
    InvalidValue(Format),
    #[error("Invalid HapType string value: `{0}`.")]
    InvalidHapTypeString(String),
    #[error("Error on value read: {0}")]
    ValueOnRead(Box<dyn std::error::Error + Send + Sync>),
    #[error("Error on value update: {0}")]
    ValueOnUpdate(Box<dyn std::error::Error + Send + Sync>),
    #[error("Error interacting with the storage.")]
    Storage,

    // converted errors
    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
    #[error("Serde JSON Error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("HTTP Status Code: {0}")]
    HttpStatus(hyper::StatusCode),
    #[error("Hyper HTTP Error: {0}")]
    Http(#[from] hyper::http::Error),
    #[error("Hyper Error: {0}")]
    Hyper(#[from] hyper::Error),
    #[error("Task Join Error: {0}")]
    TaskJoin(#[from] tokio::task::JoinError),
    #[error("AEAD Error")]
    Aead,
    #[error("HKDF Invalid Length Error")]
    HkdfInvalidLength,
    #[error("UTF-8 Error: {0}")]
    Utf8(#[from] str::Utf8Error),
    #[error("Parse EUI-48 Error: {0}")]
    ParseEui48(#[from] macaddr::ParseError),
    #[error("Parse Int Error: {0}")]
    ParseInt(#[from] num::ParseIntError),
    #[error("MPSC Send Error: {0}")]
    MpscSend(#[from] mpsc::SendError<()>),
}

impl From<aead::Error> for Error {
    fn from(_: aead::Error) -> Self { Error::Aead }
}
