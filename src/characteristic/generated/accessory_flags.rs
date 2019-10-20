// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Accessory Flags Characteristic.
pub type AccessoryFlags = Characteristic<u32>;

/// Creates a new Accessory Flags Characteristic.
pub fn new() -> AccessoryFlags {
    Characteristic::new(Inner::<u32> {
        hap_type: HapType::AccessoryFlags,
        format: Format::UInt32,
        perms: vec![Perm::PairedRead, Perm::Events],
        ..Default::default()
    })
}
