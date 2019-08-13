// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Security System Alarm Type Characteristic.
pub type SecuritySystemAlarmType = Characteristic<u8>;

/// Creates a new Security System Alarm Type Characteristic.
pub fn new() -> SecuritySystemAlarmType {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::SecuritySystemAlarmType,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		max_value: Some(1),
		min_value: Some(0),
		step_value: Some(1),
        ..Default::default()
    })
}
