// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Lock Physical Controls Characteristic.
pub type LockPhysicalControls = Characteristic<u8>;

/// Creates a new Lock Physical Controls Characteristic.
pub fn new() -> LockPhysicalControls {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::LockPhysicalControls,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        valid_values: Some(vec![
            0, // "Control Lock Disabled"
            1, // "Control Lock Enabled"
        ]),
        ..Default::default()
    })
}
