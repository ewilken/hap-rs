// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Configured Name Characteristic.
pub type ConfiguredName = Characteristic<String>;

/// Creates a new Configured Name Characteristic.
pub fn new() -> ConfiguredName {
    Characteristic::new(Inner::<String> {
        hap_type: HapType::ConfiguredName,
        format: Format::String,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
        ..Default::default()
    })
}
