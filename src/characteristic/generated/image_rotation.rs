// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm, Unit};

/// Image Rotation Characteristic.
pub type ImageRotation = Characteristic<f32>;

/// Creates a new Image Rotation Characteristic.
pub fn new() -> ImageRotation {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::ImageRotation,
        format: Format::Float,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		unit: Some(Unit::ArcDegrees),
		max_value: Some(270 as f32),
		min_value: Some(0 as f32),
		step_value: Some(90 as f32),
        ..Default::default()
    })
}
