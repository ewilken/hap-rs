// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Serial Number Characteristic.
pub type SerialNumber = Characteristic<String>;

/// Creates a new Serial Number Characteristic.
pub fn new() -> SerialNumber {
    Characteristic::new(Inner::<String> {
        hap_type: HapType::SerialNumber,
        format: Format::String,
        perms: vec![
			Perm::PairedRead,
        ],
		max_len: Some(64),
        ..Default::default()
    })
}
