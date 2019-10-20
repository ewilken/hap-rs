// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm, Unit};

/// Rotation Speed Characteristic.
pub type RotationSpeed = Characteristic<f32>;

/// Creates a new Rotation Speed Characteristic.
pub fn new() -> RotationSpeed {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::RotationSpeed,
        format: Format::Float,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        unit: Some(Unit::Percentage),
        max_value: Some(100 as f32),
        min_value: Some(0 as f32),
        step_value: Some(1 as f32),
        ..Default::default()
    })
}
