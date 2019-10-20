// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm, Unit};

/// Current Ambient Light Level Characteristic.
pub type CurrentAmbientLightLevel = Characteristic<f32>;

/// Creates a new Current Ambient Light Level Characteristic.
pub fn new() -> CurrentAmbientLightLevel {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::CurrentAmbientLightLevel,
        format: Format::Float,
        perms: vec![Perm::PairedRead, Perm::Events],
        unit: Some(Unit::Lux),
        max_value: Some(100000 as f32),
        min_value: Some(0.0001 as f32),
        ..Default::default()
    })
}
