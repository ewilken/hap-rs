// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Current Slat State Characteristic.
pub type CurrentSlatState = Characteristic<u8>;

/// Creates a new Current Slat State Characteristic.
pub fn new() -> CurrentSlatState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::CurrentSlatState,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        valid_values: Some(vec![
            0, // "Fixed"
            1, // "Jammed"
            2, // "Swinging"
        ]),
        ..Default::default()
    })
}
