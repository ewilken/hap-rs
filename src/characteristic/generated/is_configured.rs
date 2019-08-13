// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Is Configured Characteristic.
pub type IsConfigured = Characteristic<u8>;

/// Creates a new Is Configured Characteristic.
pub fn new() -> IsConfigured {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::IsConfigured,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Not Configured"
			1, // "Configured"
		]),
        ..Default::default()
    })
}
