// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Valve Type Characteristic.
pub type ValveType = Characteristic<u8>;

/// Creates a new Valve Type Characteristic.
pub fn new() -> ValveType {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::ValveType,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Generic valve"
			1, // "Irrigation"
			2, // "Shower head"
			3, // "Water faucet"
		]),
        ..Default::default()
    })
}
