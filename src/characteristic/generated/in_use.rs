// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// In Use Characteristic.
pub type InUse = Characteristic<u8>;

/// Creates a new In Use Characteristic.
pub fn new() -> InUse {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::InUse,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        valid_values: Some(vec![
            0, // "Not in use"
            1, // "In use"
        ]),
        ..Default::default()
    })
}
