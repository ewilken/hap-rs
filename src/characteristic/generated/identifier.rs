// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Identifier Characteristic.
pub type Identifier = Characteristic<u32>;

/// Creates a new Identifier Characteristic.
pub fn new() -> Identifier {
    Characteristic::new(Inner::<u32> {
        hap_type: HapType::Identifier,
        format: Format::UInt32,
        perms: vec![
			Perm::PairedRead,
        ],
		min_value: Some(0),
		step_value: Some(1),
        ..Default::default()
    })
}
