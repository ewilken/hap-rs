use crate::{
    db::{AccessoryListMember, AccessoryListPtr},
    Result,
};

pub mod bonjour;
pub mod mdns;

pub(crate) mod http;
pub(crate) mod tcp;

mod ip;

pub use self::ip::IpTransport;

/// `Transport` is implemented by the transport methods HAP supports. Currently, that's just
/// `IpTransport`.
pub trait Transport {
    /// Starts the transport.
    fn start(&mut self) -> Result<()>;
    /// Stops the transport.
    fn stop(&self) -> Result<()>;
    /// Adds an Accessory to the transport and returns a pointer to the added Accessory.
    fn add_accessory<A: 'static + AccessoryListMember + Send>(&mut self, accessory: A) -> Result<AccessoryListPtr>;
    /// Takes a pointer to an Accessory and removes the Accessory from the transport.
    fn remove_accessory(&mut self, accessory: &AccessoryListPtr) -> Result<()>;
}
