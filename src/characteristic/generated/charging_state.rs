// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Charging State Characteristic.
pub type ChargingState = Characteristic<u8>;

/// Creates a new Charging State Characteristic.
pub fn new() -> ChargingState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::ChargingState,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        valid_values: Some(vec![
            0, // "Not Charging"
            1, // "Charging"
            2, // "Not Chargeable"
        ]),
        ..Default::default()
    })
}
