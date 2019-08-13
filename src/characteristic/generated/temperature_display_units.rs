// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Temperature Display Units Characteristic.
pub type TemperatureDisplayUnits = Characteristic<u8>;

/// Creates a new Temperature Display Units Characteristic.
pub fn new() -> TemperatureDisplayUnits {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::TemperatureDisplayUnits,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Celsius"
			1, // "Fahrenheit"
		]),
        ..Default::default()
    })
}
