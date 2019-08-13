// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Target Visibility State Characteristic.
pub type TargetVisibilityState = Characteristic<u8>;

/// Creates a new Target Visibility State Characteristic.
pub fn new() -> TargetVisibilityState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::TargetVisibilityState,
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
			0, // "Shown"
			1, // "Hidden"
		]),
        ..Default::default()
    })
}
