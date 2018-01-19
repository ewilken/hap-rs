use std::io::Error;
use serde_json;

use db::{file_storage, entity};
use db::storage::Storage;

pub struct Database<S: Storage> {
    storage: S,
}

impl<S: Storage> Database<S> {
    pub fn new(storage: S) -> Database<S> {
        Database {storage: storage}
    }

    pub fn get_entity(&self, name: &str) -> Result<entity::Entity, Error> {
        let mut key = name.to_owned();
        key.push_str(".entity");
        let value: Vec<u8> = self.storage.get_byte_vec(&key)?;
        let entity = serde_json::from_slice(&value)?;
        Ok(entity)
    }

    pub fn set_entity(&self, name: &str, entity: entity::Entity) -> Result<(), Error> {
        let mut key = name.to_owned();
        key.push_str(".entity");
        let value = serde_json::to_vec(&entity)?;
        self.storage.set_byte_vec(&key, value)?;
        Ok(())
    }
}

impl Database<file_storage::FileStorage> {
    pub fn new_with_file_storage(dir: &str) -> Result<Database<file_storage::FileStorage>, Error> {
        let storage = file_storage::FileStorage::new(dir)?;
        Ok(Database {storage: storage})
    }
}
