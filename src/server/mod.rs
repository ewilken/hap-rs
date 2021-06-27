use async_trait::async_trait;
use futures::future::BoxFuture;

use crate::{accessory::HapAccessory, pointer, Result};
pub use ip::IpServer;

mod ip;

/// `Server` is implemented by the transport methods HAP supports. Currently, that's just `IpServer`.
#[async_trait]
pub trait Server {
    /// Returns a boxed Future run handle to the server that can be passed to an executor.
    fn run_handle(&self) -> BoxFuture<Result<()>>;
    /// Returns a pointer to the `Config` of the server.
    fn config_pointer(&self) -> pointer::Config;
    /// Returns a pointer to the `Storage` of the server.
    fn storage_pointer(&self) -> pointer::Storage;
    /// Adds an Accessory to the server and returns a pointer to the added Accessory.
    async fn add_accessory<A: HapAccessory + 'static>(&self, accessory: A) -> Result<pointer::Accessory>;
    /// Takes a pointer to an Accessory by reference and removes the Accessory from the server.
    async fn remove_accessory(&self, accessory: &pointer::Accessory) -> Result<()>;
    // /// Every accessory must support a manufacturer-defined mechanism to restore itself to a “factory reset” state
    // where /// all pairing information is erased and restored to factory default settings. This method is doing
    // just that. async fn factory_reset(&mut self) -> Result<()>;
}
