// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Target Fan State Characteristic.
pub type TargetFanState = Characteristic<u8>;

/// Creates a new Target Fan State Characteristic.
pub fn new() -> TargetFanState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::TargetFanState,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        valid_values: Some(vec![
            0, // "Manual"
            1, // "Auto"
        ]),
        ..Default::default()
    })
}
