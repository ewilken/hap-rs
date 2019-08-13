// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Target Media State Characteristic.
pub type TargetMediaState = Characteristic<u8>;

/// Creates a new Target Media State Characteristic.
pub fn new() -> TargetMediaState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::TargetMediaState,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		max_value: Some(2),
		min_value: Some(0),
		step_value: Some(1),
		valid_values: Some(vec![
			0, // "Play"
			1, // "Pause"
			2, // "Stop"
		]),
        ..Default::default()
    })
}
