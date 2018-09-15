use std::{rc::Rc, cell::RefCell};

use uuid::Uuid;

use db::{file_storage, storage::Storage};
use protocol::{Device, Pairing};

use Error;

/// Reference counting pointer to a `Database`.
pub type DatabasePtr = Rc<RefCell<Database>>;

/// `Database` is a wrapper type around a boxed implementor of the `Storage` trait.
pub struct Database {
    storage: Box<Storage>,
}

impl Database {
    /// Creates a new `Database`.
    pub fn new(storage: Box<Storage>) -> Database {
        Database { storage }
    }

    /// Creates a new `Database` with a `FileStorage` as its `Storage`.
    pub fn new_with_file_storage(dir: &str) -> Result<Database, Error> {
        let storage = file_storage::FileStorage::new(dir)?;
        Ok(Database::new(Box::new(storage)))
    }

    /// Returns the stored value for a given key as a `Vec<u8>`.
    pub fn get_bytes(&self, key: &str) -> Result<Vec<u8>, Error> {
        let mut k = key.to_owned();
        k.push_str(".entity");
        let value: Vec<u8> = self.storage.get_bytes(&k)?;
        Ok(value)
    }

    /// Stores a given `Vec<u8>` as the value for a given key.
    pub fn set_bytes(&self, key: &str, value: Vec<u8>) -> Result<(), Error> {
        let mut k = key.to_owned();
        k.push_str(".entity");
        self.storage.set_bytes(&k, value)?;
        Ok(())
    }

    /// Returns the stored `Device`.
    pub fn get_device(&self) -> Result<Device, Error> {
        let device_bytes = self.get_bytes("device")?;
        let device = Device::from_bytes(&device_bytes)?;
        Ok(device)
    }

    /// Stores the `Device`.
    pub fn set_device(&self, device: &Device) -> Result<(), Error> {
        let device_bytes = device.as_bytes()?;
        self.set_bytes("device", device_bytes)?;
        Ok(())
    }

    /// Returns the stored `Pairing` for a given `Uuid`.
    pub fn get_pairing(&self, id: Uuid) -> Result<Pairing, Error> {
        let pairing_bytes = self.get_bytes(&id.simple().to_string())?;
        let pairing = Pairing::from_bytes(&pairing_bytes)?;
        Ok(pairing)
    }

    /// Stores a given `Pairing`.
    pub fn set_pairing(&self, pairing: &Pairing) -> Result<(), Error> {
        let pairing_bytes = pairing.as_bytes()?;
        self.set_bytes(&pairing.id.simple().to_string(), pairing_bytes)?;
        Ok(())
    }

    /// Deletes the stored `Pairing` for a given `Uuid`.
    pub fn delete_pairing(&self, id: &Uuid) -> Result<(), Error> {
        let mut key = id.simple().to_string();
        key.push_str(".entity");
        self.storage.delete(&key)
    }

    /// Returns a `Vec` with all stored pairings.
    pub fn list_pairings(&self) -> Result<Vec<Pairing>, Error> {
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
    pub fn count_pairings(&self) -> Result<usize, Error> {
        let mut count = 0;
        for key in self.storage.keys_with_suffix("entity")? {
            if &key != "device" {
                count += 1;
            }
        }
        Ok(count)
    }
}
