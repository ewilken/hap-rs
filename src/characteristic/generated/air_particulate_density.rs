// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Air Particulate Density Characteristic.
pub type AirParticulateDensity = Characteristic<f32>;

/// Creates a new Air Particulate Density Characteristic.
pub fn new() -> AirParticulateDensity {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::AirParticulateDensity,
        format: Format::Float,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		max_value: Some(1000 as f32),
		min_value: Some(0 as f32),
		step_value: Some(1 as f32),
        ..Default::default()
    })
}
