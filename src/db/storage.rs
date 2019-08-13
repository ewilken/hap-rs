use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use uuid::Uuid;

use crate::Result;

/// `Storage` is implemented by the data storage methods HAP supports. Currently, that's just
/// `FileStorage`.
pub trait Storage {
    /// Returns a `BufReader` to the `File` stored for the given key.
    fn get_reader(&self, key: &str) -> Result<BufReader<File>>;
    /// Returns a `BufWriter` to the `File` stored for the given key.
    fn get_writer(&self, key: &str) -> Result<BufWriter<File>>;
    /// Returns the stored value for a given key as a `Vec<u8>`.
    fn get_bytes(&self, key: &str) -> Result<Vec<u8>>;
    /// Stores a given `Vec<u8>` as the value for a given key.
    fn set_bytes(&self, key: &str, value: Vec<u8>) -> Result<()>;
    /// Returns the stored value for a given key as a `u64`.
    fn get_u64(&self, key: &str) -> Result<u64>;
    /// Stores a given `u64` as the value for a given key.
    fn set_u64(&self, key: &str, value: u64) -> Result<()>;
    /// Returns the stored value for a given key as a `Uuid`.
    fn get_uuid(&self, key: &str) -> Result<Uuid>;
    /// Stores a given `Uuid` as the value for a given key.
    fn set_uuid(&self, key: &str, value: Uuid) -> Result<()>;
    /// Returns all keys with a given suffix as a `Vec<String>`.
    fn keys_with_suffix(&self, suffix: &str) -> Result<Vec<String>>;
    /// Deletes the stored value for a given key.
    fn delete(&self, key: &str) -> Result<()>;
}
