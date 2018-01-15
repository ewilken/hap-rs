extern crate eui48;
extern crate mdns;
extern crate rand;
#[macro_use]
extern crate serde_json;

pub mod accessory;
pub mod characteristic;
pub mod service;

pub mod config;
pub mod db;
pub mod hap_type;
pub mod transport;
