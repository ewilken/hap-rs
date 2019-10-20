// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm, Unit};

/// Current Horizontal Tilt Angle Characteristic.
pub type CurrentHorizontalTiltAngle = Characteristic<i32>;

/// Creates a new Current Horizontal Tilt Angle Characteristic.
pub fn new() -> CurrentHorizontalTiltAngle {
    Characteristic::new(Inner::<i32> {
        hap_type: HapType::CurrentHorizontalTiltAngle,
        format: Format::Int32,
        perms: vec![Perm::PairedRead, Perm::Events],
        unit: Some(Unit::ArcDegrees),
        max_value: Some(90),
        min_value: Some(-90),
        step_value: Some(1),
        ..Default::default()
    })
}
