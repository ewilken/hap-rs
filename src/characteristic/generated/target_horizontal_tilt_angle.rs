// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm, Unit};

/// Target Horizontal Tilt Angle Characteristic.
pub type TargetHorizontalTiltAngle = Characteristic<i32>;

/// Creates a new Target Horizontal Tilt Angle Characteristic.
pub fn new() -> TargetHorizontalTiltAngle {
    Characteristic::new(Inner::<i32> {
        hap_type: HapType::TargetHorizontalTiltAngle,
        format: Format::Int32,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		unit: Some(Unit::ArcDegrees),
		max_value: Some(90),
		min_value: Some(-90),
		step_value: Some(1),
        ..Default::default()
    })
}
