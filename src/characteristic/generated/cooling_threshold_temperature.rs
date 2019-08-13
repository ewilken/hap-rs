// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm, Unit};

/// Cooling Threshold Temperature Characteristic.
pub type CoolingThresholdTemperature = Characteristic<f32>;

/// Creates a new Cooling Threshold Temperature Characteristic.
pub fn new() -> CoolingThresholdTemperature {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::CoolingThresholdTemperature,
        format: Format::Float,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		unit: Some(Unit::Celsius),
		max_value: Some(35 as f32),
		min_value: Some(10 as f32),
		step_value: Some(0.1 as f32),
        ..Default::default()
    })
}
