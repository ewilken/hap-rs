// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm, Unit};

/// Current Media State Characteristic.
pub type CurrentMediaState = Characteristic<u8>;

/// Creates a new Current Media State Characteristic.
pub fn new() -> CurrentMediaState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::CurrentMediaState,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        unit: Some(Unit::Percentage),
        max_value: Some(3),
        min_value: Some(0),
        step_value: Some(1),
        valid_values: Some(vec![
            0, // "Play"
            1, // "Pause"
            2, // "Stop"
            3, // "Unknown"
        ]),
        ..Default::default()
    })
}
