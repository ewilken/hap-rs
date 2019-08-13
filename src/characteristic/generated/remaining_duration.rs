// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Remaining Duration Characteristic.
pub type RemainingDuration = Characteristic<u32>;

/// Creates a new Remaining Duration Characteristic.
pub fn new() -> RemainingDuration {
    Characteristic::new(Inner::<u32> {
        hap_type: HapType::RemainingDuration,
        format: Format::UInt32,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		max_value: Some(3600),
		min_value: Some(0),
		step_value: Some(1),
        ..Default::default()
    })
}
