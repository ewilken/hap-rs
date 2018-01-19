use std::io::Error;

pub trait Entity {
    fn as_byte_vec(&self) -> Result<Vec<u8>, Error>;
    fn from_byte_vec(bytes: Vec<u8>) -> Result<Self, Error>;
}
