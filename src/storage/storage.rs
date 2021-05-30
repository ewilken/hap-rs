use async_trait::async_trait;
use uuid::Uuid;

use crate::{pairing::Pairing, server::IdentifierCache, Config, Result};

/// `Storage` is implemented by the data storage methods HAP supports. Currently, that's just `FileStorage`.
#[async_trait]
pub trait Storage: Send + Sync {
    /// Loads the `Config` from the `Storage`.
    async fn load_config(&self) -> Result<Config>;
    /// Saves the `Config` into the `Storage`.
    async fn save_config(&mut self, config: &Config) -> Result<()>;
    /// Deletes the `Config` from the `Storage`.
    async fn delete_config(&mut self) -> Result<()>;
    /// Loads the `IdentifierCache` from the `Storage`.
    async fn load_identifier_cache(&self) -> Result<IdentifierCache>;
    /// Saves the `IdentifierCache` into the `Storage`.
    async fn save_identifier_cache(&mut self, identifier_cache: &IdentifierCache) -> Result<()>;
    /// Deletes the `IdentifierCache` from the `Storage`.
    async fn delete_identifier_cache(&mut self) -> Result<()>;
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
