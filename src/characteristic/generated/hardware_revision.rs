// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Hardware Revision Characteristic.
pub type HardwareRevision = Characteristic<String>;

/// Creates a new Hardware Revision Characteristic.
pub fn new() -> HardwareRevision {
    Characteristic::new(Inner::<String> {
        hap_type: HapType::HardwareRevision,
        format: Format::String,
        perms: vec![
			Perm::PairedRead,
        ],
        ..Default::default()
    })
}
