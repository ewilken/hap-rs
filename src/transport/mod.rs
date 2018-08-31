use db::{AccessoryListMember, AccessoryListPtr};

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
    /// Adds an Accessory to the transport and returns a pointer to the added Accessory.
    fn add_accessory<A: 'static + AccessoryListMember>(&mut self, accessory: A) -> Result<AccessoryListPtr, Error>;
    /// Takes a pointer to an Accessory and removes the Accessory from the transport.
    fn remove_accessory(&mut self, accessory: &AccessoryListPtr) -> Result<(), Error>;
}
