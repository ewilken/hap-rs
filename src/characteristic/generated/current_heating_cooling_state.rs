// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Current Heating Cooling State Characteristic.
pub type CurrentHeatingCoolingState = Characteristic<u8>;

/// Creates a new Current Heating Cooling State Characteristic.
pub fn new() -> CurrentHeatingCoolingState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::CurrentHeatingCoolingState,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Off"
			1, // "Heat"
			2, // "Cool"
		]),
        ..Default::default()
    })
}
