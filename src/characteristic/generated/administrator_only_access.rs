// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Administrator Only Access Characteristic.
pub type AdministratorOnlyAccess = Characteristic<bool>;

/// Creates a new Administrator Only Access Characteristic.
pub fn new() -> AdministratorOnlyAccess {
    Characteristic::new(Inner::<bool> {
        hap_type: HapType::AdministratorOnlyAccess,
        format: Format::Bool,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        ..Default::default()
    })
}
