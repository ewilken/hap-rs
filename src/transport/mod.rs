use Error;

pub mod bonjour;
pub mod mdns;

pub(crate) mod http;

mod tcp;

mod ip;

pub use self::ip::IpTransport;

/// `Transport` is implemented by the transport methods HAP supports. Currently, that's just
/// `IpTransport`.
pub trait Transport {
    /// Starts the transport.
    fn start(&mut self) -> Result<(), Error>;
    /// Stops the transport.
    fn stop(&self) -> Result<(), Error>;
}
