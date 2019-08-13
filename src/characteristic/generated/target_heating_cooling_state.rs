// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Target Heating Cooling State Characteristic.
pub type TargetHeatingCoolingState = Characteristic<u8>;

/// Creates a new Target Heating Cooling State Characteristic.
pub fn new() -> TargetHeatingCoolingState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::TargetHeatingCoolingState,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Off"
			1, // "Heat"
			2, // "Cool"
			3, // "Auto"
		]),
        ..Default::default()
    })
}
