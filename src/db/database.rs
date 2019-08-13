use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::{
    db::{file_storage, storage::Storage},
    protocol::{Device, Pairing},
};

use crate::Result;

/// Pointer to a `Database`.
pub type DatabasePtr = Arc<Mutex<Database>>;

/// `Database` is a wrapper type around a boxed implementor of the `Storage` trait.
pub struct Database {
    storage: Box<dyn Storage + Send>,
}

impl Database {
    /// Creates a new `Database`.
    pub fn new(storage: Box<dyn Storage + Send>) -> Database { Database { storage } }

    /// Creates a new `Database` with a `FileStorage` as its `Storage`.
    pub fn new_with_file_storage(dir: &str) -> Result<Database> {
        let storage = file_storage::FileStorage::new(dir)?;
        Ok(Database::new(Box::new(storage)))
    }

    /// Returns the stored value for a given key as a `Vec<u8>`.
    pub fn get_bytes(&self, key: &str) -> Result<Vec<u8>> {
        let k = format!("{}.entity", key);
        let value: Vec<u8> = self.storage.get_bytes(&k)?;
        Ok(value)
    }

    /// Stores a given `Vec<u8>` as the value for a given key.
    pub fn set_bytes(&self, key: &str, value: Vec<u8>) -> Result<()> {
        let k = format!("{}.entity", key);
        self.storage.set_bytes(&k, value)?;
        Ok(())
    }

    /// Returns the stored `Device`.
    pub fn get_device(&self) -> Result<Device> {
        let device_bytes = self.get_bytes("device")?;
        Device::from_bytes(&device_bytes)
    }

    /// Stores the `Device`.
    pub fn set_device(&self, device: &Device) -> Result<()> {
        let device_bytes = device.as_bytes()?;
        self.set_bytes("device", device_bytes)?;
        Ok(())
    }

    /// Returns the stored `Pairing` for a given `Uuid`.
    pub fn get_pairing(&self, id: Uuid) -> Result<Pairing> {
        let pairing_bytes = self.get_bytes(&id.to_simple().to_string())?;
        Pairing::from_bytes(&pairing_bytes)
    }

    /// Stores a given `Pairing`.
    pub fn set_pairing(&self, pairing: &Pairing) -> Result<()> {
        let pairing_bytes = pairing.as_bytes()?;
        self.set_bytes(&pairing.id.to_simple().to_string(), pairing_bytes)?;
        Ok(())
    }

    /// Deletes the stored `Pairing` for a given `Uuid`.
    pub fn delete_pairing(&self, id: &Uuid) -> Result<()> {
        let key = format!("{}.entity", id.to_simple().to_string());
        self.storage.delete(&key)
    }

    /// Returns a `Vec` with all stored pairings.
    pub fn list_pairings(&self) -> Result<Vec<Pairing>> {
        let mut pairings = Vec::new();
        for key in self.storage.keys_with_suffix("entity")? {
            if &key != "device" {
                let pairing_bytes = self.get_bytes(&key)?;
                let pairing = Pairing::from_bytes(&pairing_bytes)?;
                pairings.push(pairing);
            }
        }
        Ok(pairings)
    }

    /// Returns the number of stored pairings.
    pub fn count_pairings(&self) -> Result<usize> {
        let mut count = 0;
        for key in self.storage.keys_with_suffix("entity")? {
            if &key != "device" {
                count += 1;
            }
        }
        Ok(count)
    }
}
