// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Status Fault Characteristic.
pub type StatusFault = Characteristic<u8>;

/// Creates a new Status Fault Characteristic.
pub fn new() -> StatusFault {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::StatusFault,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "No Fault"
			1, // "General Fault"
		]),
        ..Default::default()
    })
}
