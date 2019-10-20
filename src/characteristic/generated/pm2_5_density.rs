// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// PM2.5 Density Characteristic.
pub type PM2_5Density = Characteristic<f32>;

/// Creates a new PM2.5 Density Characteristic.
pub fn new() -> PM2_5Density {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::PM2_5Density,
        format: Format::Float,
        perms: vec![Perm::PairedRead, Perm::Events],
        max_value: Some(1000 as f32),
        min_value: Some(0 as f32),
        step_value: Some(1 as f32),
        ..Default::default()
    })
}
