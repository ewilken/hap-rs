// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Swing Mode Characteristic.
pub type SwingMode = Characteristic<u8>;

/// Creates a new Swing Mode Characteristic.
pub fn new() -> SwingMode {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::SwingMode,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Swing Disabled"
			1, // "Swing Enabled"
		]),
        ..Default::default()
    })
}
