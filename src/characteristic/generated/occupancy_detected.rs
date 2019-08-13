// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Occupancy Detected Characteristic.
pub type OccupancyDetected = Characteristic<u8>;

/// Creates a new Occupancy Detected Characteristic.
pub fn new() -> OccupancyDetected {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::OccupancyDetected,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Occupancy Not Detected"
			1, // "Occupancy Detected"
		]),
        ..Default::default()
    })
}
