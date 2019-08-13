// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Slat Type Characteristic.
pub type SlatType = Characteristic<u8>;

/// Creates a new Slat Type Characteristic.
pub fn new() -> SlatType {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::SlatType,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
        ],
		valid_values: Some(vec![
			0, // "Horizontal"
			1, // "Vertical"
		]),
        ..Default::default()
    })
}
