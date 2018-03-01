use std::io::Error;
use uuid::Uuid;

use db::file_storage;
use db::storage::Storage;
use protocol::device::Device;
use protocol::pairing::Pairing;

pub struct Database<S: Storage> {
    storage: S,
}

impl<S: Storage> Database<S> {
    pub fn new(storage: S) -> Database<S> {
        Database {storage: storage}
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
        self.storage.delete(&id.simple().to_string())
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
}

impl Database<file_storage::FileStorage> {
    pub fn new_with_file_storage(dir: &str) -> Result<Database<file_storage::FileStorage>, Error> {
        let storage = file_storage::FileStorage::new(dir)?;
        Ok(Database {storage: storage})
    }
}
