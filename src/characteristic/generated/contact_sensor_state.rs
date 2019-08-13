// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Contact Sensor State Characteristic.
pub type ContactSensorState = Characteristic<u8>;

/// Creates a new Contact Sensor State Characteristic.
pub fn new() -> ContactSensorState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::ContactSensorState,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Contact Detected"
			1, // "Contact Not Detected"
		]),
        ..Default::default()
    })
}
