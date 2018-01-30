use std::collections::HashMap;
use std::io::Error;
use std::net::SocketAddr;
use iron::request::Request;
use serde_json;

use protocol::secured_device::SecuredDevice;

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

    pub fn get_request_address(req: &Request) -> SocketAddr {
        req.remote_addr
    }

    pub fn get_device(&self) -> Option<Vec<u8>> {
        self.get("device".into())
    }

    pub fn set_device(&mut self, secured_device: &SecuredDevice) -> Result<(), Error> {
        let device_bytes = serde_json::to_vec(secured_device.to_owned())?;
        self.set("device".into(), device_bytes);
        Ok(())
    }
}
