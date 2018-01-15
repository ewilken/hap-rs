use std::io::Error;

pub trait Storage {
    fn get(&self, key: String) -> Result<String, Error>;
    fn set(&self, key: String, value: String) -> Result<(), Error>;
    fn delete(&self, key: String) -> Result<(), Error>;
}
