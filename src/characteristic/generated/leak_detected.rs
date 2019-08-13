// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Leak Detected Characteristic.
pub type LeakDetected = Characteristic<u8>;

/// Creates a new Leak Detected Characteristic.
pub fn new() -> LeakDetected {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::LeakDetected,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Leak Not Detected"
			1, // "Leak Detected"
		]),
        ..Default::default()
    })
}
