// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Lock Current State Characteristic.
pub type LockCurrentState = Characteristic<u8>;

/// Creates a new Lock Current State Characteristic.
pub fn new() -> LockCurrentState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::LockCurrentState,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Unsecured"
			1, // "Secured"
			2, // "Jammed"
			3, // "Unknown"
		]),
        ..Default::default()
    })
}
