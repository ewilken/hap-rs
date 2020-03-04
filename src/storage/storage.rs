use uuid::Uuid;

use crate::{pairing::Pairing, Config, Result};

/// `Storage` is implemented by the data storage methods HAP supports. Currently, that's just
/// `FileStorage`.
pub trait Storage {
    /// Loads the `Config` from the `Storage`.
    fn load_config(&self) -> Result<Config>;
    /// Saves the `Config` into the `Storage`.
    fn save_config(&mut self, config: &Config) -> Result<()>;
    /// Deletes the `Config` from the `Storage`.
    fn delete_config(&mut self) -> Result<()>;
    /// Loads a `Pairing` from the `Storage`.
    fn select_pairing(&self, id: &Uuid) -> Result<Pairing>;
    /// Inserts a `Pairing` into the `Storage`.
    fn insert_pairing(&mut self, pairing: &Pairing) -> Result<()>;
    /// Deletes the `Pairing` from the `Storage`.
    fn delete_pairing(&mut self, id: &Uuid) -> Result<()>;
    /// Loads all `Pairing`s from the `Storage`.
    fn list_pairings(&self) -> Result<Vec<Pairing>>;
    /// Selects the count of stored `Pairing`s from the `Storage`.
    fn count_pairings(&self) -> Result<usize>;
}
