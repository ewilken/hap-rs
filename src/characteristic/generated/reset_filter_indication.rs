// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Reset Filter Indication Characteristic.
pub type ResetFilterIndication = Characteristic<u8>;

/// Creates a new Reset Filter Indication Characteristic.
pub fn new() -> ResetFilterIndication {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::ResetFilterIndication,
        format: Format::UInt8,
        perms: vec![Perm::PairedWrite],
        max_value: Some(1),
        min_value: Some(1),
        step_value: Some(1),
        ..Default::default()
    })
}
