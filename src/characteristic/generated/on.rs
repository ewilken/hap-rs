// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// On Characteristic.
pub type On = Characteristic<bool>;

/// Creates a new On Characteristic.
pub fn new() -> On {
    Characteristic::new(Inner::<bool> {
        hap_type: HapType::On,
        format: Format::Bool,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        ..Default::default()
    })
}
