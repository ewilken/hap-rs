// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Security System Target State Characteristic.
pub type SecuritySystemTargetState = Characteristic<u8>;

/// Creates a new Security System Target State Characteristic.
pub fn new() -> SecuritySystemTargetState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::SecuritySystemTargetState,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        valid_values: Some(vec![
            0, // "Stay Arm"
            1, // "Away Arm"
            2, // "Night Arm"
            3, // "Disarm"
        ]),
        ..Default::default()
    })
}
