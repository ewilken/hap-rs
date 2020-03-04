use futures::future::BoxFuture;

use crate::{pointer, storage::accessory_list::AccessoryListMember, Result};

mod ip;

pub use ip::IpServer;

/// `Server` is implemented by the transport methods HAP supports. Currently, that's just `IpServer`.
pub trait Server {
    /// Returns a boxed Future run handle to the server that can be passed to an executor.
    fn run_handle(&self) -> BoxFuture<()>;
    /// Returns a pointer to the `Config` of the server.
    fn config_pointer(&self) -> pointer::Config;
    /// Returns a pointer to the `Storage` of the server.
    fn storage_pointer(&self) -> pointer::Storage;
    /// Adds an Accessory to the server and returns a pointer to the added Accessory.
    fn add_accessory<A: 'static + AccessoryListMember + Send>(
        &mut self,
        accessory: A,
    ) -> Result<pointer::AccessoryListMember>;
    /// Takes a pointer to an Accessory by reference and removes the Accessory from the server.
    fn remove_accessory(&mut self, accessory: &pointer::AccessoryListMember) -> Result<()>;
}
