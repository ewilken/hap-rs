// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Firmware Revision Characteristic.
pub type FirmwareRevision = Characteristic<String>;

/// Creates a new Firmware Revision Characteristic.
pub fn new() -> FirmwareRevision {
    Characteristic::new(Inner::<String> {
        hap_type: HapType::FirmwareRevision,
        format: Format::String,
        perms: vec![Perm::PairedRead],
        ..Default::default()
    })
}
