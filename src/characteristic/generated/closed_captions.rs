// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Closed Captions Characteristic.
pub type ClosedCaptions = Characteristic<u8>;

/// Creates a new Closed Captions Characteristic.
pub fn new() -> ClosedCaptions {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::ClosedCaptions,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		max_value: Some(1),
		min_value: Some(0),
		step_value: Some(1),
		valid_values: Some(vec![
			0, // "Disabled"
			1, // "Enabled"
		]),
        ..Default::default()
    })
}
