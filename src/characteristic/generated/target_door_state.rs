// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Target Door State Characteristic.
pub type TargetDoorState = Characteristic<u8>;

/// Creates a new Target Door State Characteristic.
pub fn new() -> TargetDoorState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::TargetDoorState,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Open"
			1, // "Closed"
		]),
        ..Default::default()
    })
}
