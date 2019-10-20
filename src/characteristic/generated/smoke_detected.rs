// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Smoke Detected Characteristic.
pub type SmokeDetected = Characteristic<u8>;

/// Creates a new Smoke Detected Characteristic.
pub fn new() -> SmokeDetected {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::SmokeDetected,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        valid_values: Some(vec![
            0, // "Smoke Not Detected"
            1, // "Smoke Detected"
        ]),
        ..Default::default()
    })
}
