// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Night Vision Characteristic.
pub type NightVision = Characteristic<bool>;

/// Creates a new Night Vision Characteristic.
pub fn new() -> NightVision {
    Characteristic::new(Inner::<bool> {
        hap_type: HapType::NightVision,
        format: Format::Bool,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        ..Default::default()
    })
}
