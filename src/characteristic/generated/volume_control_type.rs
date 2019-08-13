// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Volume Control Type Characteristic.
pub type VolumeControlType = Characteristic<u8>;

/// Creates a new Volume Control Type Characteristic.
pub fn new() -> VolumeControlType {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::VolumeControlType,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		max_value: Some(3),
		min_value: Some(0),
		step_value: Some(1),
		valid_values: Some(vec![
			0, // "None"
			1, // "Relative"
			2, // "RelativeWithCurrent"
			3, // "Absolute"
		]),
        ..Default::default()
    })
}
