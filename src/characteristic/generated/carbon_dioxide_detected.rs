// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Carbon Dioxide Detected Characteristic.
pub type CarbonDioxideDetected = Characteristic<u8>;

/// Creates a new Carbon Dioxide Detected Characteristic.
pub fn new() -> CarbonDioxideDetected {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::CarbonDioxideDetected,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        valid_values: Some(vec![
            0, // "CO2 Levels Normal"
            1, // "CO2 Levels Abnormal"
        ]),
        ..Default::default()
    })
}
