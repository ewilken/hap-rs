// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Active Identifier Characteristic.
pub type ActiveIdentifier = Characteristic<u32>;

/// Creates a new Active Identifier Characteristic.
pub fn new() -> ActiveIdentifier {
    Characteristic::new(Inner::<u32> {
        hap_type: HapType::ActiveIdentifier,
        format: Format::UInt32,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		min_value: Some(0),
        ..Default::default()
    })
}
