use std::{fmt, marker::PhantomData};

use crypto::ed25519;
use rand::{self, Rng};
use serde::{
    de::{self, Deserialize, Deserializer, SeqAccess, Visitor},
    ser::{Serialize, SerializeTuple, Serializer},
};
use serde_derive::{Deserialize, Serialize};
use serde_json;

use crate::{
    db::{Database, DatabasePtr},
    pin::Pin,
    Error,
};

/// `Device` represents instances of the HAP server.
#[derive(Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub pin: Pin,
    #[serde(with = "BigArray")]
    pub private_key: [u8; 64],
    pub public_key: [u8; 32],
}

impl Device {
    /// Creates a new `Device` with a given key pair.
    pub fn new(id: String, pin: Pin, private_key: [u8; 64], public_key: [u8; 32]) -> Device {
        Device {
            id,
            pin,
            public_key,
            private_key,
        }
    }

    /// Creates a new `Device` generating a random key pair.
    pub fn new_random(id: String, pin: Pin) -> Device {
        let (private_key, public_key) = generate_key_pair();
        Device {
            id,
            pin,
            private_key,
            public_key,
        }
    }

    /// Attempts to load a `Device` from a database and creates a new one with a random key pair if
    /// none is found for the given ID.
    pub fn load_or_new(id: String, pin: Pin, database: &Database) -> Result<Device, Error> {
        match database.get_device() {
            Ok(device) => Ok(device),
            Err(_) => {
                let device = Device::new_random(id, pin);
                database.set_device(&device)?;
                Ok(device)
            },
        }
    }

    /// Loads a `Device` from a database.
    pub fn load_from(database: &DatabasePtr) -> Result<Device, Error> {
        database.lock().expect("couldn't access database").get_device()
    }

    /// Saves a `Device` to a database.
    pub fn save_to(&self, database: &DatabasePtr) -> Result<(), Error> {
        database.lock().expect("couldn't access database").set_device(self)?;
        Ok(())
    }

    /// Serializes a `Device` to a `Vec<u8>`.
    pub fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        let value = serde_json::to_vec(&self)?;
        Ok(value)
    }

    /// Deserializes a `Device` from a `&[u8]`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Device, Error> {
        let value = serde_json::from_slice(bytes)?;
        Ok(value)
    }
}

fn generate_key_pair() -> ([u8; 64], [u8; 32]) {
    let mut rng = rand::thread_rng();
    let seed = rng.gen::<[u8; 32]>();
    ed25519::keypair(&seed)
}

// see https://github.com/serde-rs/serde/issues/631
trait BigArray<'de>: Sized {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

macro_rules! big_array {
    ($($len:expr,)+) => {
        $(
            impl<'de, T> BigArray<'de> for [T; $len] where T: Default + Copy + Serialize + Deserialize<'de> {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                    let mut seq = serializer.serialize_tuple(self.len())?;
                    for elem in &self[..] {
                        seq.serialize_element(elem)?;
                    }
                    seq.end()
                }

                fn deserialize<D>(deserializer: D) -> Result<[T; $len], D::Error> where D: Deserializer<'de> {
                    struct ArrayVisitor<T> {
                        element: PhantomData<T>,
                    }

                    impl<'de, T> Visitor<'de> for ArrayVisitor<T> where T: Default + Copy + Deserialize<'de> {
                        type Value = [T; $len];

                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            formatter.write_str(concat!("an array of length ", $len))
                        }

                        fn visit_seq<A>(self, mut seq: A) -> Result<[T; $len], A::Error> where A: SeqAccess<'de> {
                            let mut arr = [T::default(); $len];
                            for i in 0..$len {
                                arr[i] = seq.next_element()?
                                    .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                            }
                            Ok(arr)
                        }
                    }

                    let visitor = ArrayVisitor {element: PhantomData};
                    deserializer.deserialize_tuple($len, visitor)
                }
            }
        )+
    }
}

big_array! {
    40, 48, 50, 56, 64, 72, 96, 100, 128, 160, 192, 200, 224, 256, 384, 512,
    768, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
}
