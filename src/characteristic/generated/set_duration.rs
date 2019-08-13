// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Set Duration Characteristic.
pub type SetDuration = Characteristic<u32>;

/// Creates a new Set Duration Characteristic.
pub fn new() -> SetDuration {
    Characteristic::new(Inner::<u32> {
        hap_type: HapType::SetDuration,
        format: Format::UInt32,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		max_value: Some(3600),
		min_value: Some(0),
		step_value: Some(1),
        ..Default::default()
    })
}
