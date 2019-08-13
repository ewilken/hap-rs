// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Identify Characteristic.
pub type Identify = Characteristic<bool>;

/// Creates a new Identify Characteristic.
pub fn new() -> Identify {
    Characteristic::new(Inner::<bool> {
        hap_type: HapType::Identify,
        format: Format::Bool,
        perms: vec![
			Perm::PairedWrite,
        ],
        ..Default::default()
    })
}
