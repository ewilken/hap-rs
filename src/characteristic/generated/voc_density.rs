// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// VOC Density Characteristic.
pub type VOCDensity = Characteristic<f32>;

/// Creates a new VOC Density Characteristic.
pub fn new() -> VOCDensity {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::VOCDensity,
        format: Format::Float,
        perms: vec![Perm::PairedRead, Perm::Events],
        max_value: Some(1000 as f32),
        min_value: Some(0 as f32),
        step_value: Some(1 as f32),
        ..Default::default()
    })
}
