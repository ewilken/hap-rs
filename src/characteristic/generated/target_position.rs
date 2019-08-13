// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm, Unit};

/// Target Position Characteristic.
pub type TargetPosition = Characteristic<u8>;

/// Creates a new Target Position Characteristic.
pub fn new() -> TargetPosition {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::TargetPosition,
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
