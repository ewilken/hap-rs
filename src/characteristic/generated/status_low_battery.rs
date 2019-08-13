// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Status Low Battery Characteristic.
pub type StatusLowBattery = Characteristic<u8>;

/// Creates a new Status Low Battery Characteristic.
pub fn new() -> StatusLowBattery {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::StatusLowBattery,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Battery Level Normal"
			1, // "Battery Level Low"
		]),
        ..Default::default()
    })
}
