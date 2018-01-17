use std::io::Error;

pub trait Storage {
    fn get(&self, key: &str) -> Result<Vec<u8>, Error>;
    fn set(&self, key: &str, value: Vec<u8>) -> Result<(), Error>;
    fn delete(&self, key: &str) -> Result<(), Error>;
}
