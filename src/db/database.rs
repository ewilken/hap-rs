use std::io::Error;

use db::file_storage;
use db::storage::Storage;

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
}

impl Database<file_storage::FileStorage> {
    pub fn new_with_file_storage(dir: &str) -> Result<Database<file_storage::FileStorage>, Error> {
        let storage = file_storage::FileStorage::new(dir)?;
        Ok(Database {storage: storage})
    }
}
