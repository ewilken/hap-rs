// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm, Unit};

/// Current Position Characteristic.
pub type CurrentPosition = Characteristic<u8>;

/// Creates a new Current Position Characteristic.
pub fn new() -> CurrentPosition {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::CurrentPosition,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        unit: Some(Unit::Percentage),
        max_value: Some(100),
        min_value: Some(0),
        step_value: Some(1),
        ..Default::default()
    })
}
