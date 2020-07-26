use async_trait::async_trait;
use uuid::Uuid;

use crate::{pairing::Pairing, server::ServerPersistence, Config, Result};

/// `Storage` is implemented by the data storage methods HAP supports. Currently, that's just `FileStorage`.
#[async_trait]
pub trait Storage: Send + Sync {
    /// Loads the `Config` from the `Storage`.
    async fn load_config(&self) -> Result<Config>;
    /// Saves the `Config` into the `Storage`.
    async fn save_config(&mut self, config: &Config) -> Result<()>;
    /// Deletes the `Config` from the `Storage`.
    async fn delete_config(&mut self) -> Result<()>;
    /// Loads the `ServerPersistence` from the `Storage`.
    async fn load_server_persistence(&self) -> Result<ServerPersistence>;
    /// Saves the `ServerPersistence` into the `Storage`.
    async fn save_server_persistence(&mut self, server_persistence: &ServerPersistence) -> Result<()>;
    /// Deletes the `ServerPersistence` from the `Storage`.
    async fn delete_server_persistence(&mut self) -> Result<()>;
    /// Loads a `Pairing` from the `Storage`.
    async fn load_pairing(&self, id: &Uuid) -> Result<Pairing>;
    /// Inserts a `Pairing` into the `Storage`.
    async fn save_pairing(&mut self, pairing: &Pairing) -> Result<()>;
    /// Deletes the `Pairing` from the `Storage`.
    async fn delete_pairing(&mut self, id: &Uuid) -> Result<()>;
    /// Loads all `Pairing`s from the `Storage`.
    async fn list_pairings(&self) -> Result<Vec<Pairing>>;
    /// Selects the count of stored `Pairing`s from the `Storage`.
    async fn count_pairings(&self) -> Result<usize>;
}
