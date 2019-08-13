// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Current Fan State Characteristic.
pub type CurrentFanState = Characteristic<u8>;

/// Creates a new Current Fan State Characteristic.
pub fn new() -> CurrentFanState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::CurrentFanState,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Inactive"
			1, // "Idle"
			2, // "Blowing Air"
		]),
        ..Default::default()
    })
}
