// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Program Mode Characteristic.
pub type ProgramMode = Characteristic<u8>;

/// Creates a new Program Mode Characteristic.
pub fn new() -> ProgramMode {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::ProgramMode,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "No program scheduled"
			1, // "Program scheduled"
			2, // "Program scheduled (Manual Mode)"
		]),
        ..Default::default()
    })
}
