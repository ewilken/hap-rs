// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Outlet In Use Characteristic.
pub type OutletInUse = Characteristic<bool>;

/// Creates a new Outlet In Use Characteristic.
pub fn new() -> OutletInUse {
    Characteristic::new(Inner::<bool> {
        hap_type: HapType::OutletInUse,
        format: Format::Bool,
        perms: vec![Perm::PairedRead, Perm::Events],
        ..Default::default()
    })
}
