// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Volume Selector Characteristic.
pub type VolumeSelector = Characteristic<u8>;

/// Creates a new Volume Selector Characteristic.
pub fn new() -> VolumeSelector {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::VolumeSelector,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedWrite,
        ],
		max_value: Some(1),
		min_value: Some(0),
		step_value: Some(1),
		valid_values: Some(vec![
			0, // "Increment"
			1, // "Decrement"
		]),
        ..Default::default()
    })
}
