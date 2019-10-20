// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Air Particulate Size Characteristic.
pub type AirParticulateSize = Characteristic<u8>;

/// Creates a new Air Particulate Size Characteristic.
pub fn new() -> AirParticulateSize {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::AirParticulateSize,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        valid_values: Some(vec![
            0, // "2.5 μm"
            1, // "10 μm"
        ]),
        ..Default::default()
    })
}
