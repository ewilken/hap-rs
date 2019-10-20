// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Target Air Quality Characteristic.
pub type TargetAirQuality = Characteristic<u8>;

/// Creates a new Target Air Quality Characteristic.
pub fn new() -> TargetAirQuality {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::TargetAirQuality,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        valid_values: Some(vec![
            0, // "Excellent"
            1, // "Good"
            2, // "Fair"
        ]),
        ..Default::default()
    })
}
