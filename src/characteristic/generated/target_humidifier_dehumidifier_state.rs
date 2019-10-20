// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Target Humidifier Dehumidifier State Characteristic.
pub type TargetHumidifierDehumidifierState = Characteristic<u8>;

/// Creates a new Target Humidifier Dehumidifier State Characteristic.
pub fn new() -> TargetHumidifierDehumidifierState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::TargetHumidifierDehumidifierState,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        valid_values: Some(vec![
            0, // "Humidifier or Dehumidifier"
            1, // "Humidifier"
            2, // "Dehumidifier"
        ]),
        ..Default::default()
    })
}
