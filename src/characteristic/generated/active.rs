// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Active Characteristic.
pub type Active = Characteristic<u8>;

/// Creates a new Active Characteristic.
pub fn new() -> Active {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::Active,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Inactive"
			1, // "Active"
		]),
        ..Default::default()
    })
}
