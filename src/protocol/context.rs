use std::collections::HashMap;
use std::sync::Mutex;
use std::net::SocketAddr;
use iron::request::Request;

use protocol::secured_device::SecuredDevice;

pub struct Context {
    storage: Mutex<HashMap<Vec<u8>, Vec<u8>>>,
}

impl Context {
    pub fn get(&self, key: Vec<u8>) -> Option<&Vec<u8>> {
        let storage = self.storage.lock().unwrap();
        storage.get(&key)
    }

    pub fn set(&self, key: Vec<u8>, value: Vec<u8>) {
        let storage = self.storage.lock().unwrap();
        storage.insert(key, value);
    }

    pub fn delete(&self, key: Vec<u8>) {
        let storage = self.storage.lock().unwrap();
        storage.remove(&key);
    }

    pub fn get_connection_key(req: &Request) -> SocketAddr {
        req.remote_addr
    }

    pub fn get_secured_device(&self) -> Option<&Vec<u8>> {
        self.get("device".into())
    }

    /*pub fn set_secured_device(&self, secured_device: SecuredDevice) {
        self.set("device".into(), secured_device);
    }*/
}
