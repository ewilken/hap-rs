use rand::{self, Rng};
use crypto::ed25519;
use std::fmt;
use std::marker::PhantomData;
use std::io::Error;
use std::sync::{Arc, Mutex};
use serde::ser::{Serialize, Serializer, SerializeTuple};
use serde::de;
use serde::de::{Deserialize, Deserializer, Visitor, SeqAccess};
use serde_json;

use pin::Pin;
use db::database::Database;
use db::file_storage::FileStorage;
use db::context::Context;
use db::storage::Storage;

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub pin: Pin,
    #[serde(with = "BigArray")]
    pub private_key: [u8; 64],
    pub public_key: [u8; 32],
}

impl Device {
    pub fn new(id: String, pin: Pin, private_key: [u8; 64], public_key: [u8; 32]) -> Device {
        Device {id: id.to_owned(), pin, public_key, private_key}
    }

    pub fn new_random(id: String, pin: Pin) -> Device {
        let (private_key, public_key) = generate_key_pair();
        Device {id: id.to_owned(), pin, private_key, public_key}
    }

    pub fn load_or_new(id: String, pin: Pin, database: &Database<FileStorage>) -> Result<Device, Error> {
        if let Some(device) = database.get_device().ok() {
            return Ok(device)
        }
        let device = Device::new_random(id, pin);
        database.set_device(&device)?;
        Ok(device)
    }

    pub fn load<S: Storage>(context: &Arc<Mutex<Context>>, database: &Arc<Mutex<Database<S>>>) -> Result<Device, Error> {
        let mut c = context.lock().unwrap();
        if let Some(device) = c.get_device().ok() {
            return Ok(device);
        }
        let d = database.lock().unwrap();
        match d.get_device() {
            Ok(device) => {
                c.set_device(&device)?;
                Ok(device)
            },
            Err(err) => Err(err),
        }
    }

    pub fn save<S: Storage>(&self, context: &Arc<Mutex<Context>>, database: &Arc<Mutex<Database<S>>>) -> Result<(), Error> {
        let d = database.lock().unwrap();
        d.set_device(self)?;
        let mut c = context.lock().unwrap();
        c.set_device(self)?;
        Ok(())
    }

    pub fn as_byte_vec(&self) -> Result<Vec<u8>, Error> {
        let value = serde_json::to_vec(&self)?;
        Ok(value)
    }

    pub fn from_byte_vec(bytes: Vec<u8>) -> Result<Device, Error> {
        let value = serde_json::from_slice(&bytes)?;
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
        where S: Serializer;
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>;
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
