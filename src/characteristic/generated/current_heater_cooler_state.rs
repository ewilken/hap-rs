// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Current Heater Cooler State Characteristic.
pub type CurrentHeaterCoolerState = Characteristic<u8>;

/// Creates a new Current Heater Cooler State Characteristic.
pub fn new() -> CurrentHeaterCoolerState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::CurrentHeaterCoolerState,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        valid_values: Some(vec![
            0, // "Inactive"
            1, // "Idle"
            2, // "Heating"
            3, // "Cooling"
        ]),
        ..Default::default()
    })
}
