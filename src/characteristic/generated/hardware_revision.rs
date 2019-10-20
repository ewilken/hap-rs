// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Hardware Revision Characteristic.
pub type HardwareRevision = Characteristic<String>;

/// Creates a new Hardware Revision Characteristic.
pub fn new() -> HardwareRevision {
    Characteristic::new(Inner::<String> {
        hap_type: HapType::HardwareRevision,
        format: Format::String,
        perms: vec![Perm::PairedRead],
        ..Default::default()
    })
}
