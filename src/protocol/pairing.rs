use std::io::Error;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use serde_json;

use db::database::Database;
use db::context::Context;
use db::storage::Storage;

#[derive(Serialize, Deserialize)]
pub struct Pairing {
    pub id: Uuid,
    pub public_key: [u8; 32],
}

impl Pairing {
    pub fn new(id: Uuid, public_key: [u8; 32]) -> Pairing {
        Pairing {id, public_key}
    }

    pub fn load<S: Storage>(id: Uuid, database: &Arc<Mutex<Database<S>>>) -> Result<Pairing, Error> {
        let d = database.lock().unwrap();
        d.get_pairing(id)
    }

    pub fn save<S: Storage>(&self, database: &Arc<Mutex<Database<S>>>) -> Result<(), Error> {
        let d = database.lock().unwrap();
        d.set_pairing(self)?;
        Ok(())
    }

    pub fn as_byte_vec(&self) -> Result<Vec<u8>, Error> {
        let value = serde_json::to_vec(&self)?;
        Ok(value)
    }

    pub fn from_byte_vec(bytes: Vec<u8>) -> Result<Pairing, Error> {
        let value = serde_json::from_slice(&bytes)?;
        Ok(value)
    }
}
