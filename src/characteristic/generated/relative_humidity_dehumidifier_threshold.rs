// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm, Unit};

/// Relative Humidity Dehumidifier Threshold Characteristic.
pub type RelativeHumidityDehumidifierThreshold = Characteristic<f32>;

/// Creates a new Relative Humidity Dehumidifier Threshold Characteristic.
pub fn new() -> RelativeHumidityDehumidifierThreshold {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::RelativeHumidityDehumidifierThreshold,
        format: Format::Float,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        unit: Some(Unit::Percentage),
        max_value: Some(100 as f32),
        min_value: Some(0 as f32),
        step_value: Some(1 as f32),
        ..Default::default()
    })
}
