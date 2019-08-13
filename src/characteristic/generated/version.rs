// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Version Characteristic.
pub type Version = Characteristic<String>;

/// Creates a new Version Characteristic.
pub fn new() -> Version {
    Characteristic::new(Inner::<String> {
        hap_type: HapType::Version,
        format: Format::String,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		max_len: Some(64),
        ..Default::default()
    })
}
