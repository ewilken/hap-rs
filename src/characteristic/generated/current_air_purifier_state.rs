// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Current Air Purifier State Characteristic.
pub type CurrentAirPurifierState = Characteristic<u8>;

/// Creates a new Current Air Purifier State Characteristic.
pub fn new() -> CurrentAirPurifierState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::CurrentAirPurifierState,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Inactive"
			1, // "Idle"
			2, // "Purifying Air"
		]),
        ..Default::default()
    })
}
