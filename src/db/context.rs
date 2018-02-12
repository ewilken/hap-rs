use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use uuid::Uuid;

use protocol::device::Device;
use protocol::pairing::Pairing;

pub struct Context {
    storage: HashMap<Vec<u8>, Vec<u8>>,
}

impl Context {
    pub fn new() -> Context {
        let storage: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();
        Context {storage}
    }

    pub fn get(&self, key: Vec<u8>) -> Option<Vec<u8>> {
        if let Some(value) = self.storage.get(&key) {
            return Some(value.to_owned());
        }
        None
    }

    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.storage.insert(key, value);
    }

    pub fn delete(&mut self, key: Vec<u8>) {
        self.storage.remove(&key);
    }

    pub fn get_device(&self) -> Result<Device, Error> {
        let device_bytes = self.get("device".into()).ok_or(Error::new(ErrorKind::Other, "no device in context"))?;
        let device = Device::from_byte_vec(device_bytes)?;
        Ok(device)
    }

    pub fn set_device(&mut self, device: &Device) -> Result<(), Error> {
        let device_bytes = device.as_byte_vec()?;
        self.set("device".into(), device_bytes);
        Ok(())
    }

    pub fn get_pairing(&self, id: Uuid) -> Result<Pairing, Error> {
        let pairing_bytes = self.get(id.as_bytes().to_vec()).ok_or(Error::new(ErrorKind::Other, "no pairing in context"))?;
        let pairing = Pairing::from_byte_vec(pairing_bytes)?;
        Ok(pairing)
    }

    pub fn set_pairing(&mut self, pairing: &Pairing) -> Result<(), Error> {
        let pairing_bytes = pairing.as_byte_vec()?;
        self.set(pairing.id.as_bytes().to_vec(), pairing_bytes);
        Ok(())
    }
}
