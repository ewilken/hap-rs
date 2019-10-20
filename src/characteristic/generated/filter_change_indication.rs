// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Filter Change Indication Characteristic.
pub type FilterChangeIndication = Characteristic<u8>;

/// Creates a new Filter Change Indication Characteristic.
pub fn new() -> FilterChangeIndication {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::FilterChangeIndication,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        valid_values: Some(vec![
            0, // "Filter OK"
            1, // "Change Filter"
        ]),
        ..Default::default()
    })
}
