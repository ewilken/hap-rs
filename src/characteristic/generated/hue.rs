// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm, Unit};

/// Hue Characteristic.
pub type Hue = Characteristic<f32>;

/// Creates a new Hue Characteristic.
pub fn new() -> Hue {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::Hue,
        format: Format::Float,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        unit: Some(Unit::ArcDegrees),
        max_value: Some(360 as f32),
        min_value: Some(0 as f32),
        step_value: Some(1 as f32),
        ..Default::default()
    })
}
