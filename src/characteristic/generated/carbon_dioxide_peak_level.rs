// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Carbon Dioxide Peak Level Characteristic.
pub type CarbonDioxidePeakLevel = Characteristic<f32>;

/// Creates a new Carbon Dioxide Peak Level Characteristic.
pub fn new() -> CarbonDioxidePeakLevel {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::CarbonDioxidePeakLevel,
        format: Format::Float,
        perms: vec![Perm::PairedRead, Perm::Events],
        max_value: Some(100000 as f32),
        min_value: Some(0 as f32),
        ..Default::default()
    })
}
