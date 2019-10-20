// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Carbon Monoxide Peak Level Characteristic.
pub type CarbonMonoxidePeakLevel = Characteristic<f32>;

/// Creates a new Carbon Monoxide Peak Level Characteristic.
pub fn new() -> CarbonMonoxidePeakLevel {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::CarbonMonoxidePeakLevel,
        format: Format::Float,
        perms: vec![Perm::PairedRead, Perm::Events],
        max_value: Some(100 as f32),
        min_value: Some(0 as f32),
        ..Default::default()
    })
}
