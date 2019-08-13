// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm, Unit};

/// Volume Characteristic.
pub type Volume = Characteristic<u8>;

/// Creates a new Volume Characteristic.
pub fn new() -> Volume {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::Volume,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		unit: Some(Unit::Percentage),
		max_value: Some(100),
		min_value: Some(0),
		step_value: Some(1),
        ..Default::default()
    })
}
