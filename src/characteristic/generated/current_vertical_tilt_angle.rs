// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm, Unit};

/// Current Vertical Tilt Angle Characteristic.
pub type CurrentVerticalTiltAngle = Characteristic<i32>;

/// Creates a new Current Vertical Tilt Angle Characteristic.
pub fn new() -> CurrentVerticalTiltAngle {
    Characteristic::new(Inner::<i32> {
        hap_type: HapType::CurrentVerticalTiltAngle,
        format: Format::Int32,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		unit: Some(Unit::ArcDegrees),
		max_value: Some(90),
		min_value: Some(-90),
		step_value: Some(1),
        ..Default::default()
    })
}
