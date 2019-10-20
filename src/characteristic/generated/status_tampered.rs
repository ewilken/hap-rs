// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Status Tampered Characteristic.
pub type StatusTampered = Characteristic<u8>;

/// Creates a new Status Tampered Characteristic.
pub fn new() -> StatusTampered {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::StatusTampered,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        valid_values: Some(vec![
            0, // "Not Tampered"
            1, // "Tampered"
        ]),
        ..Default::default()
    })
}
