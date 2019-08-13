// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Status Active Characteristic.
pub type StatusActive = Characteristic<bool>;

/// Creates a new Status Active Characteristic.
pub fn new() -> StatusActive {
    Characteristic::new(Inner::<bool> {
        hap_type: HapType::StatusActive,
        format: Format::Bool,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
        ..Default::default()
    })
}
