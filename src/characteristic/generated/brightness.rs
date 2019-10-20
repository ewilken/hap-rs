// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm, Unit};

/// Brightness Characteristic.
pub type Brightness = Characteristic<i32>;

/// Creates a new Brightness Characteristic.
pub fn new() -> Brightness {
    Characteristic::new(Inner::<i32> {
        hap_type: HapType::Brightness,
        format: Format::Int32,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        unit: Some(Unit::Percentage),
        max_value: Some(100),
        min_value: Some(0),
        step_value: Some(1),
        ..Default::default()
    })
}
