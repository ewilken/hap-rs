use crate::{Error, Result};

pub type Pin = String;

const INVALID_PINS: [&'static str; 12] = [
    "12345678", "87654321", "00000000", "11111111", "22222222", "33333333", "44444444", "55555555", "66666666",
    "77777777", "88888888", "99999999",
];

pub fn new(input: &str) -> Result<Pin> {
    if INVALID_PINS.contains(&input) {
        return Err(Error::from_str("invalid pin - too easy"));
    }
    if input.chars().count() != 8 {
        return Err(Error::from_str("pin must be 8 characters long"));
    }
    for digit in input.chars() {
        if digit < '0' || digit > '9' {
            return Err(Error::from_str("pin must only contain numbers"));
        }
    }

    Ok(format!("{}-{}-{}", &input[..3], &input[3..5], &input[5..]))
}
