// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Identify Characteristic.
pub type Identify = Characteristic<bool>;

/// Creates a new Identify Characteristic.
pub fn new() -> Identify {
    Characteristic::new(Inner::<bool> {
        hap_type: HapType::Identify,
        format: Format::Bool,
        perms: vec![Perm::PairedWrite],
        ..Default::default()
    })
}
