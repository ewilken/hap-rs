use std::{io::Error, sync::{Arc, Mutex}};

use uuid::Uuid;

use db::{file_storage, storage::Storage};
use protocol::{device::Device, pairing::Pairing};

pub type DatabasePtr = Arc<Mutex<Database>>;

pub struct Database {
    storage: Box<Storage>,
}

impl Database {
    pub fn new(storage: Box<Storage>) -> Database {
        Database { storage }
    }

    pub fn new_with_file_storage(dir: &str) -> Result<Database, Error> {
        let storage = file_storage::FileStorage::new(dir)?;
        Ok(Database::new(Box::new(storage)))
    }

    pub fn get_byte_vec(&self, name: &str) -> Result<Vec<u8>, Error> {
        let mut key = name.to_owned();
        key.push_str(".entity");
        let value: Vec<u8> = self.storage.get_byte_vec(&key)?;
        Ok(value)
    }

    pub fn set_byte_vec(&self, name: &str, value: Vec<u8>) -> Result<(), Error> {
        let mut key = name.to_owned();
        key.push_str(".entity");
        self.storage.set_byte_vec(&key, value)?;
        Ok(())
    }

    pub fn get_device(&self) -> Result<Device, Error> {
        let device_bytes = self.get_byte_vec("device")?;
        let device = Device::from_byte_vec(device_bytes)?;
        Ok(device)
    }

    pub fn set_device(&self, device: &Device) -> Result<(), Error> {
        let device_bytes = device.as_byte_vec()?;
        self.set_byte_vec("device", device_bytes)?;
        Ok(())
    }

    pub fn get_pairing(&self, id: Uuid) -> Result<Pairing, Error> {
        let pairing_bytes = self.get_byte_vec(&id.simple().to_string())?;
        let pairing = Pairing::from_byte_vec(pairing_bytes)?;
        Ok(pairing)
    }

    pub fn set_pairing(&self, pairing: &Pairing) -> Result<(), Error> {
        let pairing_bytes = pairing.as_byte_vec()?;
        self.set_byte_vec(&pairing.id.simple().to_string(), pairing_bytes)?;
        Ok(())
    }

    pub fn delete_pairing(&self, id: &Uuid) -> Result<(), Error> {
        let mut key = id.simple().to_string();
        key.push_str(".entity");
        self.storage.delete(&key)
    }

    pub fn list_pairings(&self) -> Result<Vec<Pairing>, Error> {
        let mut pairings = Vec::new();
        for key in self.storage.keys_with_suffix("entity")? {
            if key != String::from("device") {
                let pairing_bytes = self.get_byte_vec(&key)?;
                let pairing = Pairing::from_byte_vec(pairing_bytes)?;
                pairings.push(pairing);
            }
        }
        Ok(pairings)
    }

    pub fn count_pairings(&self) -> Result<usize, Error> {
        let mut count = 0;
        for key in self.storage.keys_with_suffix("entity")? {
            if key != String::from("device") {
                count += 1;
            }
        }
        Ok(count)
    }
}
