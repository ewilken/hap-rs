// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm, Unit};

/// Battery Level Characteristic.
pub type BatteryLevel = Characteristic<u8>;

/// Creates a new Battery Level Characteristic.
pub fn new() -> BatteryLevel {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::BatteryLevel,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        unit: Some(Unit::Percentage),
        max_value: Some(100),
        min_value: Some(0),
        step_value: Some(1),
        ..Default::default()
    })
}
