use Error;

pub mod bonjour;
pub mod http;
pub mod mdns;
pub mod tcp;

mod ip;

pub use self::ip::IpTransport;

pub trait Transport {
    fn start(&mut self) -> Result<(), Error>;
    fn stop(&self) -> Result<(), Error>;
}
