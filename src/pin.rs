use crate::Error;

pub type Pin = String;

pub fn new(input: &str) -> Result<Pin, Error> {
    let invalid_pins: [String; 12] = [
        "12345678".into(),
        "87654321".into(),
        "00000000".into(),
        "11111111".into(),
        "22222222".into(),
        "33333333".into(),
        "44444444".into(),
        "55555555".into(),
        "66666666".into(),
        "77777777".into(),
        "88888888".into(),
        "99999999".into(),
    ];
    for invalid_pin in &invalid_pins {
        if input == invalid_pin {
            return Err(Error::from_str("invalid pin - too easy"));
        }
    }
    if input.chars().count() != 8 {
        return Err(Error::from_str("pin must be 8 characters long"));
    }
    for digit in input.chars() {
        if digit < '0' || digit > '9' {
            return Err(Error::from_str("pin must only contain numbers"));
        }
    }

    let mut pin = String::from(&input[..3]);
    pin.push_str("-");
    pin.push_str(&input[3..5]);
    pin.push_str("-");
    pin.push_str(&input[5..]);

    Ok(pin)
}
