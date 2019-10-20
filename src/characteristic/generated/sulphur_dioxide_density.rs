// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Sulphur Dioxide Density Characteristic.
pub type SulphurDioxideDensity = Characteristic<f32>;

/// Creates a new Sulphur Dioxide Density Characteristic.
pub fn new() -> SulphurDioxideDensity {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::SulphurDioxideDensity,
        format: Format::Float,
        perms: vec![Perm::PairedRead, Perm::Events],
        max_value: Some(1000 as f32),
        min_value: Some(0 as f32),
        step_value: Some(1 as f32),
        ..Default::default()
    })
}
