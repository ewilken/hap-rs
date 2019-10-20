// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm, Unit};

/// Target Vertical Tilt Angle Characteristic.
pub type TargetVerticalTiltAngle = Characteristic<i32>;

/// Creates a new Target Vertical Tilt Angle Characteristic.
pub fn new() -> TargetVerticalTiltAngle {
    Characteristic::new(Inner::<i32> {
        hap_type: HapType::TargetVerticalTiltAngle,
        format: Format::Int32,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        unit: Some(Unit::ArcDegrees),
        max_value: Some(90),
        min_value: Some(-90),
        step_value: Some(1),
        ..Default::default()
    })
}
