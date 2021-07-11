use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{Error, Result};

/// A [`Pairing`](Pairing) represents a paired controller.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pairing {
    pub id: Uuid,
    pub permissions: Permissions,
    pub public_key: [u8; 32],
}

impl Pairing {
    /// Creates a new [`Pairing`](Pairing).
    pub fn new(id: Uuid, permissions: Permissions, public_key: [u8; 32]) -> Pairing {
        Pairing {
            id,
            permissions,
            public_key,
        }
    }

    /// Deserializes a [`Pairing`](Pairing) from bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Pairing> {
        let value = serde_json::from_slice(&bytes)?;
        Ok(value)
    }

    /// Serializes a [`Pairing`](Pairing) to bytes.
    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        let value = serde_json::to_vec(&self)?;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairing_from_bytes() {
        let pairing = Pairing {
            id: Uuid::parse_str("bc158b86-cabf-432d-aee4-422ef0e3f1d5").unwrap(),
            permissions: Permissions::Admin,
            public_key: [
                215, 90, 152, 1, 130, 177, 10, 183, 213, 75, 254, 211, 201, 100, 7, 58, 14, 225, 114, 243, 218, 166,
                35, 37, 175, 2, 26, 104, 247, 7, 81, 26,
            ],
        };
        assert_eq!(
            Pairing::from_bytes(&b"{\"id\":\"bc158b86-cabf-432d-aee4-422ef0e3f1d5\",\"permissions\":\"0x01\",\"public_key\":[215,90,152,1,130,177,10,183,213,75,254,211,201,100,7,58,14,225,114,243,218,166,35,37,175,2,26,104,247,7,81,26]}".to_vec()).unwrap(),
            pairing
        );
    }

    #[test]
    fn test_pairing_to_bytes() {
        let pairing = Pairing {
            id: Uuid::parse_str("bc158b86-cabf-432d-aee4-422ef0e3f1d5").unwrap(),
            permissions: Permissions::User,
            public_key: [
                215, 90, 152, 1, 130, 177, 10, 183, 213, 75, 254, 211, 201, 100, 7, 58, 14, 225, 114, 243, 218, 166,
                35, 37, 175, 2, 26, 104, 247, 7, 81, 26,
            ],
        };
        assert_eq!(
            pairing.as_bytes().unwrap(),
            b"{\"id\":\"bc158b86-cabf-432d-aee4-422ef0e3f1d5\",\"permissions\":\"0x00\",\"public_key\":[215,90,152,1,130,177,10,183,213,75,254,211,201,100,7,58,14,225,114,243,218,166,35,37,175,2,26,104,247,7,81,26]}".to_vec()
        );
    }
}

/// The permissions of a paired controller.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Permissions {
    #[serde(rename = "0x00")]
    User,
    /// Admins are pairings that have the admin bit set. Admins are exclusively authorized to add, remove, and list
    /// pairings.
    #[serde(rename = "0x01")]
    Admin,
}

impl Permissions {
    /// Converts a Byte value to the corresponding `Permissions` variant.
    pub fn from_byte(byte: u8) -> Result<Permissions> {
        match byte {
            0x00 => Ok(Permissions::User),
            0x01 => Ok(Permissions::Admin),
            _ => Err(Error::InvalidPairingPermission(byte)),
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
