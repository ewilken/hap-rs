// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm, Unit};

/// Saturation Characteristic.
pub type Saturation = Characteristic<f32>;

/// Creates a new Saturation Characteristic.
pub fn new() -> Saturation {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::Saturation,
        format: Format::Float,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        unit: Some(Unit::Percentage),
        max_value: Some(100 as f32),
        min_value: Some(0 as f32),
        step_value: Some(1 as f32),
        ..Default::default()
    })
}
