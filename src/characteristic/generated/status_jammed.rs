// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Status Jammed Characteristic.
pub type StatusJammed = Characteristic<u8>;

/// Creates a new Status Jammed Characteristic.
pub fn new() -> StatusJammed {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::StatusJammed,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        valid_values: Some(vec![
            0, // "Not Jammed"
            1, // "Jammed"
        ]),
        ..Default::default()
    })
}
