// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Current Visibility State Characteristic.
pub type CurrentVisibilityState = Characteristic<u8>;

/// Creates a new Current Visibility State Characteristic.
pub fn new() -> CurrentVisibilityState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::CurrentVisibilityState,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		max_value: Some(3),
		min_value: Some(0),
		step_value: Some(1),
		valid_values: Some(vec![
			0, // "Shown"
			1, // "Hidden"
		]),
        ..Default::default()
    })
}
