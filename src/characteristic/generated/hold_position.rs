// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Hold Position Characteristic.
pub type HoldPosition = Characteristic<bool>;

/// Creates a new Hold Position Characteristic.
pub fn new() -> HoldPosition {
    Characteristic::new(Inner::<bool> {
        hap_type: HapType::HoldPosition,
        format: Format::Bool,
        perms: vec![
			Perm::PairedWrite,
        ],
        ..Default::default()
    })
}
