// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Target Heater Cooler State Characteristic.
pub type TargetHeaterCoolerState = Characteristic<u8>;

/// Creates a new Target Heater Cooler State Characteristic.
pub fn new() -> TargetHeaterCoolerState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::TargetHeaterCoolerState,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        valid_values: Some(vec![
            0, // "Auto"
            1, // "Heat"
            2, // "Cool"
        ]),
        ..Default::default()
    })
}
