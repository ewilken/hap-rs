// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Target Air Purifier State Characteristic.
pub type TargetAirPurifierState = Characteristic<u8>;

/// Creates a new Target Air Purifier State Characteristic.
pub fn new() -> TargetAirPurifierState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::TargetAirPurifierState,
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
