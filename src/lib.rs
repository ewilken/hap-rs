pub mod accessory;
pub mod characteristic;
pub mod service;

pub mod db;
pub mod protocol;
pub mod transport;

mod config;
mod error;
mod event;
mod hap_type;
mod pin;

pub use crate::{
    config::Config,
    error::{Error, ErrorKind},
    hap_type::HapType,
};

pub type Result<T> = std::result::Result<T, Error>;
