// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Ozone Density Characteristic.
pub type OzoneDensity = Characteristic<f32>;

/// Creates a new Ozone Density Characteristic.
pub fn new() -> OzoneDensity {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::OzoneDensity,
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
