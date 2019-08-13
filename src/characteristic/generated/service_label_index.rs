// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Service Label Index Characteristic.
pub type ServiceLabelIndex = Characteristic<u8>;

/// Creates a new Service Label Index Characteristic.
pub fn new() -> ServiceLabelIndex {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::ServiceLabelIndex,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
        ],
		max_value: Some(255),
		min_value: Some(1),
		step_value: Some(1),
        ..Default::default()
    })
}
