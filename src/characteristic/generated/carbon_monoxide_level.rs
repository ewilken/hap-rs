// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Carbon Monoxide Level Characteristic.
pub type CarbonMonoxideLevel = Characteristic<f32>;

/// Creates a new Carbon Monoxide Level Characteristic.
pub fn new() -> CarbonMonoxideLevel {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::CarbonMonoxideLevel,
        format: Format::Float,
        perms: vec![Perm::PairedRead, Perm::Events],
        max_value: Some(100 as f32),
        min_value: Some(0 as f32),
        ..Default::default()
    })
}
