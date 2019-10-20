// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm, Unit};

/// Target Temperature Characteristic.
pub type TargetTemperature = Characteristic<f32>;

/// Creates a new Target Temperature Characteristic.
pub fn new() -> TargetTemperature {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::TargetTemperature,
        format: Format::Float,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        unit: Some(Unit::Celsius),
        max_value: Some(38 as f32),
        min_value: Some(10 as f32),
        step_value: Some(0.1 as f32),
        ..Default::default()
    })
}
