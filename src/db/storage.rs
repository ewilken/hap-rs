use std::{fs::File, io::{BufReader, BufWriter}};

use uuid::Uuid;

use Error;

pub trait Storage {
    fn get_reader(&self, key: &str) -> Result<BufReader<File>, Error>;
    fn get_writer(&self, key: &str) -> Result<BufWriter<File>, Error>;
    fn get_byte_vec(&self, key: &str) -> Result<Vec<u8>, Error>;
    fn set_byte_vec(&self, key: &str, value: Vec<u8>) -> Result<(), Error>;
    fn get_u64(&self, key: &str) -> Result<u64, Error>;
    fn set_u64(&self, key: &str, value: u64) -> Result<(), Error>;
    fn get_uuid(&self, key: &str) -> Result<Uuid, Error>;
    fn set_uuid(&self, key: &str, value: Uuid) -> Result<(), Error>;
    fn keys_with_suffix(&self, suffix: &str) -> Result<Vec<String>, Error>;
    fn delete(&self, key: &str) -> Result<(), Error>;
}
