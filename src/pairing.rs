use ed25519_dalek::PublicKey;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Error, Result};

/// `Pairing` represents paired controllers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pairing {
    pub id: Uuid,
    pub permissions: Permissions,
    pub public_key: PublicKey,
}

impl Pairing {
    /// Creates a new `Pairing`.
    pub fn new(id: Uuid, permissions: Permissions, public_key: PublicKey) -> Pairing {
        Pairing {
            id,
            permissions,
            public_key,
        }
    }

    /// Deserializes a `Pairing` from bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Pairing> {
        let value = serde_json::from_slice(&bytes)?;
        Ok(value)
    }

    /// Serializes a `Pairing` to bytes.
    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        let value = serde_json::to_vec(&self)?;
        Ok(value)
    }
}

/// The permissions of a paired controller.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Permissions {
    #[serde(rename = "0x00")]
    User,
    #[serde(rename = "0x01")]
    Admin,
}

impl Permissions {
    /// Converts a Byte value to the corresponding `Permissions` variant.
    pub fn from_byte(byte: u8) -> Result<Permissions> {
        match byte {
            0x00 => Ok(Permissions::User),
            0x01 => Ok(Permissions::Admin),
            _ => Err(Error::from_str("invalid permission Byte")),
        }
    }

    /// Converts a `Permissions` variant to the corresponding Byte value.
    pub fn as_byte(&self) -> u8 {
        match *self {
            Permissions::User => 0x00,
            Permissions::Admin => 0x01,
        }
    }
}
