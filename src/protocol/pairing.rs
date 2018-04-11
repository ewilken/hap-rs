use std::io::Error;

use uuid::Uuid;
use serde_json;

use db::database::DatabasePtr;

#[derive(Serialize, Deserialize)]
pub struct Pairing {
    pub id: Uuid,
    pub permissions: Permissions,
    pub public_key: [u8; 32],
}

impl Pairing {
    pub fn new(id: Uuid, permissions: Permissions, public_key: [u8; 32]) -> Pairing {
        Pairing {id, permissions, public_key}
    }

    pub fn load(id: Uuid, database: &DatabasePtr) -> Result<Pairing, Error> {
        let d = database.lock().unwrap();
        d.get_pairing(id)
    }

    pub fn save(&self, database: &DatabasePtr) -> Result<(), Error> {
        let d = database.lock().unwrap();
        d.set_pairing(self)?;
        Ok(())
    }

    pub fn as_byte_vec(&self) -> Result<Vec<u8>, Error> {
        let value = serde_json::to_vec(&self)?;
        Ok(value)
    }

    pub fn from_byte_vec(bytes: Vec<u8>) -> Result<Pairing, Error> {
        let value = serde_json::from_slice(&bytes)?;
        Ok(value)
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum Permissions {
    #[serde(rename = "0x00")]
    User,
    #[serde(rename = "0x01")]
    Admin,
}

impl Permissions {
    pub fn from_u8(u: u8) -> Result<Permissions, ()> {
        match u {
            0x00 => Ok(Permissions::User),
            0x01 => Ok(Permissions::Admin),
            _ => Err(())
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            &Permissions::User => 0x00,
            &Permissions::Admin => 0x01,
        }
    }
}
