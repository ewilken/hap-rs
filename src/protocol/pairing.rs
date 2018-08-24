use std::{rc::Rc, cell::RefCell};

use uuid::Uuid;
use serde_json;

use db::DatabasePtr;

use Error;

/// `Pairing` represents paired controllers.
#[derive(Debug, Serialize, Deserialize)]
pub struct Pairing {
    pub id: Uuid,
    pub permissions: Permissions,
    pub public_key: [u8; 32],
}

impl Pairing {
    /// Creates a new `Pairing`.
    pub fn new(id: Uuid, permissions: Permissions, public_key: [u8; 32]) -> Pairing {
        Pairing {id, permissions, public_key}
    }

    /// Loads a `Pairing` from a database.
    pub fn load_from(id: Uuid, database: &DatabasePtr) -> Result<Pairing, Error> {
        database.try_borrow()?.get_pairing(id)
    }

    /// Saves a `Pairing` to a database.
    pub fn save_to(&self, database: &DatabasePtr) -> Result<(), Error> {
        database.try_borrow_mut()?.set_pairing(self)?;
        Ok(())
    }

    /// Serializes a `Pairing` to a `Vec<u8>`.
    pub fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        let value = serde_json::to_vec(&self)?;
        Ok(value)
    }

    /// Deserializes a `Pairing` from a `Vec<u8>`.
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Pairing, Error> {
        let value = serde_json::from_slice(&bytes)?;
        Ok(value)
    }
}

/// The permissions of a paired controller.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Permissions {
    #[serde(rename = "0x00")]
    User,
    #[serde(rename = "0x01")]
    Admin,
}

impl Permissions {
    /// Converts a Byte value to the corresponding `Permissions` variant.
    pub fn from_u8(u: u8) -> Result<Permissions, Error> {
        match u {
            0x00 => Ok(Permissions::User),
            0x01 => Ok(Permissions::Admin),
            _ => Err(Error::new_io("invalid permission Byte"))
        }
    }

    /// Converts a `Permissions` variant to the corresponding Byte value.
    pub fn as_u8(&self) -> u8 {
        match self {
            &Permissions::User => 0x00,
            &Permissions::Admin => 0x01,
        }
    }
}

/// Reference counting pointer to a `Uuid`.
pub type IdPtr = Rc<RefCell<Option<Uuid>>>;
