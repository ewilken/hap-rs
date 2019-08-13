// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Remote Key Characteristic.
pub type RemoteKey = Characteristic<u8>;

/// Creates a new Remote Key Characteristic.
pub fn new() -> RemoteKey {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::RemoteKey,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedWrite,
        ],
		max_value: Some(16),
		min_value: Some(0),
		step_value: Some(1),
		valid_values: Some(vec![
			0, // "Rewind"
			1, // "FastForward"
			10, // "Exit"
			11, // "PlayPause"
			15, // "Info"
			2, // "NextTrack"
			3, // "PrevTrack"
			4, // "ArrowUp"
			5, // "ArrowDown"
			6, // "ArrowLeft"
			7, // "ArrowRight"
			8, // "Select"
			9, // "Back"
		]),
        ..Default::default()
    })
}
