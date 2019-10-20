// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm, Unit};

/// Water Level Characteristic.
pub type WaterLevel = Characteristic<f32>;

/// Creates a new Water Level Characteristic.
pub fn new() -> WaterLevel {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::WaterLevel,
        format: Format::Float,
        perms: vec![Perm::PairedRead, Perm::Events],
        unit: Some(Unit::Percentage),
        max_value: Some(100 as f32),
        min_value: Some(0 as f32),
        ..Default::default()
    })
}
