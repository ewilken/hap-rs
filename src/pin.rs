use serde::{Deserialize, Serialize};

use crate::{Error, Result};

const INVALID_PINS: [[u8; 8]; 12] = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1, 1, 1, 1],
    [2, 2, 2, 2, 2, 2, 2, 2],
    [3, 3, 3, 3, 3, 3, 3, 3],
    [4, 4, 4, 4, 4, 4, 4, 4],
    [5, 5, 5, 5, 5, 5, 5, 5],
    [6, 6, 6, 6, 6, 6, 6, 6],
    [7, 7, 7, 7, 7, 7, 7, 7],
    [8, 8, 8, 8, 8, 8, 8, 8],
    [9, 9, 9, 9, 9, 9, 9, 9],
    [1, 2, 3, 4, 5, 6, 7, 8],
    [8, 7, 6, 5, 4, 3, 2, 1],
];

/// The `Pin` struct represents the server's 8 digit pin used for pairing.
///
/// The pin consists of eight digits between 0 and 9 and defaults to `11122333`.
///
/// The following pins are considered too easy and are therefore not allowed:
/// - `00000000`
/// - `11111111`
/// - `22222222`
/// - `33333333`
/// - `44444444`
/// - `55555555`
/// - `66666666`
/// - `77777777`
/// - `88888888`
/// - `99999999`
/// - `12345678`
/// - `87654321`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pin {
    pin: [u8; 8],
}

impl Pin {
    /// Creates a new `Pin`.
    pub fn new(pin: [u8; 8]) -> Result<Self> {
        if INVALID_PINS.contains(&pin) {
            return Err(Error::PinTooEasy);
        }
        for digit in &pin {
            if digit > &9 {
                return Err(Error::InvalidPin);
            }
        }

        Ok(Pin { pin })
    }

    // TODO: fix UTF-8 encoding here
    // pub fn as_bytes(&self) -> [u8; 10] {
    //     [
    //         self.pin[0],
    //         self.pin[1],
    //         self.pin[2],
    //         45, // '-'
    //         self.pin[3],
    //         self.pin[4],
    //         45, // '-'
    //         self.pin[5],
    //         self.pin[6],
    //         self.pin[7],
    //     ]
    // }
}

impl ToString for Pin {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}-{}{}-{}{}{}",
            &self.pin[0],
            &self.pin[1],
            &self.pin[2],
            &self.pin[3],
            &self.pin[4],
            &self.pin[5],
            &self.pin[6],
            &self.pin[7],
        )
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    // #[test]
    // fn test_invalid_pin() {
    //     let too_easy_pin = Pin::new([1, 2, 3, 4, 5, 6, 7, 8]);
    //     let pin_with_invalid_number = Pin::new([0, 0, 0, 0, 0, 0, 0, 123]);
    // }

    #[test]
    fn test_to_string() {
        let pin = Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap();
        assert_eq!(pin.to_string(), "111-22-333".to_string());
    }

    // #[test]
    // fn test_as_bytes() {
    //     let pin = Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap();
    //     let bytes = pin.as_bytes();
    //     assert_eq!(bytes, "111-22-333".to_string().as_bytes());
    // }
}
