// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Target Slat State Characteristic.
pub type TargetSlatState = Characteristic<u8>;

/// Creates a new Target Slat State Characteristic.
pub fn new() -> TargetSlatState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::TargetSlatState,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Manual"
			1, // "Auto"
		]),
        ..Default::default()
    })
}
