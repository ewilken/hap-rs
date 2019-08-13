// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Security System Current State Characteristic.
pub type SecuritySystemCurrentState = Characteristic<u8>;

/// Creates a new Security System Current State Characteristic.
pub fn new() -> SecuritySystemCurrentState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::SecuritySystemCurrentState,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Stay Arm"
			1, // "Away Arm"
			2, // "Night Arm"
			3, // "Disarmed"
			4, // "Alarm Triggered"
		]),
        ..Default::default()
    })
}
