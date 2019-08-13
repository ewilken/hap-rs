// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm, Unit};

/// Current Relative Humidity Characteristic.
pub type CurrentRelativeHumidity = Characteristic<f32>;

/// Creates a new Current Relative Humidity Characteristic.
pub fn new() -> CurrentRelativeHumidity {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::CurrentRelativeHumidity,
        format: Format::Float,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		unit: Some(Unit::Percentage),
		max_value: Some(100 as f32),
		min_value: Some(0 as f32),
		step_value: Some(1 as f32),
        ..Default::default()
    })
}
