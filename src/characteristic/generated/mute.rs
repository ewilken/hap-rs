// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Mute Characteristic.
pub type Mute = Characteristic<bool>;

/// Creates a new Mute Characteristic.
pub fn new() -> Mute {
    Characteristic::new(Inner::<bool> {
        hap_type: HapType::Mute,
        format: Format::Bool,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        ..Default::default()
    })
}
