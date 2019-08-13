// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm, Unit};

/// Target Relative Humidity Characteristic.
pub type TargetRelativeHumidity = Characteristic<f32>;

/// Creates a new Target Relative Humidity Characteristic.
pub fn new() -> TargetRelativeHumidity {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::TargetRelativeHumidity,
        format: Format::Float,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		unit: Some(Unit::Percentage),
		max_value: Some(100 as f32),
		min_value: Some(0 as f32),
		step_value: Some(1 as f32),
        ..Default::default()
    })
}
