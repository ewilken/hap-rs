// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Lock Target State Characteristic.
pub type LockTargetState = Characteristic<u8>;

/// Creates a new Lock Target State Characteristic.
pub fn new() -> LockTargetState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::LockTargetState,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Unsecured"
			1, // "Secured"
		]),
        ..Default::default()
    })
}
