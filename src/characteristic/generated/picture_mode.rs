// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Picture Mode Characteristic.
pub type PictureMode = Characteristic<u16>;

/// Creates a new Picture Mode Characteristic.
pub fn new() -> PictureMode {
    Characteristic::new(Inner::<u16> {
        hap_type: HapType::PictureMode,
        format: Format::UInt16,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		max_value: Some(13),
		min_value: Some(0),
		step_value: Some(1),
		valid_values: Some(vec![
			0, // "Other"
			1, // "Standard"
			2, // "Calibrated"
			3, // "CalibratedDark"
			4, // "Vivid"
			5, // "Game"
			6, // "Computer"
			7, // "Custom"
		]),
        ..Default::default()
    })
}
