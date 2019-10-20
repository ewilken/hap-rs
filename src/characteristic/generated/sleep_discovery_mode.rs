// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Sleep Discovery Mode Characteristic.
pub type SleepDiscoveryMode = Characteristic<u8>;

/// Creates a new Sleep Discovery Mode Characteristic.
pub fn new() -> SleepDiscoveryMode {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::SleepDiscoveryMode,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        max_value: Some(1),
        min_value: Some(0),
        valid_values: Some(vec![
            0, // "NotDiscoverable"
            1, // "AlwaysDiscoverable"
        ]),
        ..Default::default()
    })
}
