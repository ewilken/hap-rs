// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm, Unit};

/// Target Tilt Angle Characteristic.
pub type TargetTiltAngle = Characteristic<i32>;

/// Creates a new Target Tilt Angle Characteristic.
pub fn new() -> TargetTiltAngle {
    Characteristic::new(Inner::<i32> {
        hap_type: HapType::TargetTiltAngle,
        format: Format::Int32,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        unit: Some(Unit::ArcDegrees),
        max_value: Some(90),
        min_value: Some(-90),
        step_value: Some(1),
        ..Default::default()
    })
}
