// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Rotation Direction Characteristic.
pub type RotationDirection = Characteristic<i32>;

/// Creates a new Rotation Direction Characteristic.
pub fn new() -> RotationDirection {
    Characteristic::new(Inner::<i32> {
        hap_type: HapType::RotationDirection,
        format: Format::Int32,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Clockwise"
			1, // "Counter-clockwise"
		]),
        ..Default::default()
    })
}
