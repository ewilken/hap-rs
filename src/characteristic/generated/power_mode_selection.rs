// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Power Mode Selection Characteristic.
pub type PowerModeSelection = Characteristic<u8>;

/// Creates a new Power Mode Selection Characteristic.
pub fn new() -> PowerModeSelection {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::PowerModeSelection,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedWrite,
        ],
		max_value: Some(1),
		min_value: Some(0),
		step_value: Some(1),
		valid_values: Some(vec![
			0, // "Show"
			1, // "Hide"
		]),
        ..Default::default()
    })
}
