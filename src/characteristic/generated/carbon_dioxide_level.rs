// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Carbon Dioxide Level Characteristic.
pub type CarbonDioxideLevel = Characteristic<f32>;

/// Creates a new Carbon Dioxide Level Characteristic.
pub fn new() -> CarbonDioxideLevel {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::CarbonDioxideLevel,
        format: Format::Float,
        perms: vec![Perm::PairedRead, Perm::Events],
        max_value: Some(100000 as f32),
        min_value: Some(0 as f32),
        ..Default::default()
    })
}
