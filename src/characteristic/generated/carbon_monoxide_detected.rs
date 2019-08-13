// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Carbon Monoxide Detected Characteristic.
pub type CarbonMonoxideDetected = Characteristic<u8>;

/// Creates a new Carbon Monoxide Detected Characteristic.
pub fn new() -> CarbonMonoxideDetected {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::CarbonMonoxideDetected,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "CO Levels Normal"
			1, // "CO Levels Abnormal"
		]),
        ..Default::default()
    })
}
