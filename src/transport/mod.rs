use std::io::Error;

pub mod bonjour;
pub mod ip;
pub mod http;
pub mod mdns;
pub mod tlv;
pub mod accessory_list;

pub trait Transport {
    fn start(&mut self) -> Result<(), Error>;
    fn stop(&self) -> Result<(), Error>;
}
