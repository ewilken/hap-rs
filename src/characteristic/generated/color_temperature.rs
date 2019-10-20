// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Color Temperature Characteristic.
pub type ColorTemperature = Characteristic<u32>;

/// Creates a new Color Temperature Characteristic.
pub fn new() -> ColorTemperature {
    Characteristic::new(Inner::<u32> {
        hap_type: HapType::ColorTemperature,
        format: Format::UInt32,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        max_value: Some(500),
        min_value: Some(140),
        step_value: Some(1),
        ..Default::default()
    })
}
