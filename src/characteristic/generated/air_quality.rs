// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Air Quality Characteristic.
pub type AirQuality = Characteristic<u8>;

/// Creates a new Air Quality Characteristic.
pub fn new() -> AirQuality {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::AirQuality,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        valid_values: Some(vec![
            0, // "Unknown"
            1, // "Excellent"
            2, // "Good"
            3, // "Fair"
            4, // "Inferior"
            5, // "Poor"
        ]),
        ..Default::default()
    })
}
