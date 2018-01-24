use std::io::Error;

pub mod bonjour;
pub mod ip;
pub mod http;
pub mod mdns;

pub trait Transport {
    fn start(&self) -> Result<(), Error>;
    fn stop(&self) -> Result<(), Error>;
}
