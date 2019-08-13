// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Current Humidifier Dehumidifier State Characteristic.
pub type CurrentHumidifierDehumidifierState = Characteristic<u8>;

/// Creates a new Current Humidifier Dehumidifier State Characteristic.
pub fn new() -> CurrentHumidifierDehumidifierState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::CurrentHumidifierDehumidifierState,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Inactive"
			1, // "Idle"
			2, // "Humidifying"
			3, // "Dehumidifying"
		]),
        ..Default::default()
    })
}
