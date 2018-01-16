use std::io::Error;

use db::{storage, file_storage};

pub struct Database<S: storage::Storage> {
    storage: S,
}

impl<S: storage::Storage> Database<S> {
    fn new(storage: S) -> Database<S> {
        Database {storage: storage}
    }

    fn new_with_file_storage(dir: &str) -> Result<Database<file_storage::FileStorage>, Error> {
        let storage = file_storage::FileStorage::new(dir)?;
        Ok(Database {storage: storage})
    }
}
