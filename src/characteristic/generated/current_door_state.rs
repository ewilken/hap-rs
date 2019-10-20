// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Current Door State Characteristic.
pub type CurrentDoorState = Characteristic<u8>;

/// Creates a new Current Door State Characteristic.
pub fn new() -> CurrentDoorState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::CurrentDoorState,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        valid_values: Some(vec![
            0, // "Open"
            1, // "Closed"
            2, // "Opening"
            3, // "Closing"
            4, // "Stopped"
        ]),
        ..Default::default()
    })
}
