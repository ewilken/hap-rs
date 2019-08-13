// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm, Unit};

/// Current Tilt Angle Characteristic.
pub type CurrentTiltAngle = Characteristic<i32>;

/// Creates a new Current Tilt Angle Characteristic.
pub fn new() -> CurrentTiltAngle {
    Characteristic::new(Inner::<i32> {
        hap_type: HapType::CurrentTiltAngle,
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
