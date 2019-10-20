// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm, Unit};

/// Heating Threshold Temperature Characteristic.
pub type HeatingThresholdTemperature = Characteristic<f32>;

/// Creates a new Heating Threshold Temperature Characteristic.
pub fn new() -> HeatingThresholdTemperature {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::HeatingThresholdTemperature,
        format: Format::Float,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        unit: Some(Unit::Celsius),
        max_value: Some(25 as f32),
        min_value: Some(0 as f32),
        step_value: Some(0.1 as f32),
        ..Default::default()
    })
}
