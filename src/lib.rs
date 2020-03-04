mod config;
mod error;
mod event;
mod hap_type;
mod pin;
mod pointer;
mod tlv;
mod transport;

pub mod accessory;
pub mod characteristic;
pub mod service;

pub mod pairing;
pub mod server;
pub mod storage;

pub use crate::{
    config::Config,
    error::{Error, ErrorKind},
    hap_type::HapType,
    pin::Pin,
    transport::bonjour::{BonjourFeatureFlag, BonjourStatusFlag},
};

pub type Result<T> = std::result::Result<T, Error>;

pub use tokio;
