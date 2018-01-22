use std::io::Error;

pub mod bonjour;
pub mod ip;
pub mod http;

pub trait Transport {
    fn start() -> Result<(), Error>;
    fn stop() -> Result<(), Error>;
}
