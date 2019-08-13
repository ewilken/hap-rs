// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// PM10 Density Characteristic.
pub type PM10Density = Characteristic<f32>;

/// Creates a new PM10 Density Characteristic.
pub fn new() -> PM10Density {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::PM10Density,
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
