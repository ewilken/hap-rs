pub use ed25519_dalek::Keypair as Ed25519Keypair;
pub use futures;
pub use macaddr::MacAddr6 as MacAddress;
pub use serde_json;

pub use crate::{
    config::Config,
    error::Error,
    hap_type::HapType,
    pin::Pin,
    transport::bonjour::{BonjourFeatureFlag, BonjourStatusFlag},
};

mod config;
mod error;
mod event;
mod hap_type;
mod pin;
pub mod pointer;
mod tlv;
mod transport;

/// Definitions of HomeKit accessories.
pub mod accessory;
/// Definitions of HomeKit characteristics.
pub mod characteristic;
/// Representation of paired controllers.
pub mod pairing;
/// The HomeKit Accessory Server implementation.
pub mod server;
/// Definitions of HomeKit services.
pub mod service;
/// Representations of persistent storage.
pub mod storage;

/// `Result` type redefinition.
pub type Result<T> = std::result::Result<T, Error>;
