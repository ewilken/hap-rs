// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Filter Life Level Characteristic.
pub type FilterLifeLevel = Characteristic<f32>;

/// Creates a new Filter Life Level Characteristic.
pub fn new() -> FilterLifeLevel {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::FilterLifeLevel,
        format: Format::Float,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		max_value: Some(100 as f32),
		min_value: Some(0 as f32),
        ..Default::default()
    })
}
