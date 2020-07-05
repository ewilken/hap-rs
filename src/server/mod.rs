use async_trait::async_trait;
use futures::future::BoxFuture;

use crate::{accessory::HapAccessory, pointer, Result};

mod ip;

pub use ip::IpServer;

/// `Server` is implemented by the transport methods HAP supports. Currently, that's just `IpServer`.
#[async_trait]
pub trait Server {
    /// Returns a boxed Future run handle to the server that can be passed to an executor.
    fn run_handle(&self) -> BoxFuture<()>;
    /// Returns a pointer to the `Config` of the server.
    fn config_pointer(&self) -> pointer::Config;
    /// Returns a pointer to the `Storage` of the server.
    fn storage_pointer(&self) -> pointer::Storage;
    /// Adds an Accessory to the server and returns a pointer to the added Accessory.
    async fn add_accessory<A: HapAccessory + 'static>(&mut self, accessory: A) -> Result<pointer::Accessory>;
    /// Takes a pointer to an Accessory by reference and removes the Accessory from the server.
    async fn remove_accessory(&mut self, accessory: &pointer::Accessory) -> Result<()>;
}
