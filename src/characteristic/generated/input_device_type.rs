// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Input Device Type Characteristic.
pub type InputDeviceType = Characteristic<u8>;

/// Creates a new Input Device Type Characteristic.
pub fn new() -> InputDeviceType {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::InputDeviceType,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		max_value: Some(5),
		min_value: Some(0),
		step_value: Some(1),
		valid_values: Some(vec![
			0, // "Other"
			1, // "Tv"
			2, // "Recording"
			3, // "Tuner"
			4, // "Playback"
			5, // "AudioSystem"
		]),
        ..Default::default()
    })
}
