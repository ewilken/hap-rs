// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Service Label Namespace Characteristic.
pub type ServiceLabelNamespace = Characteristic<u8>;

/// Creates a new Service Label Namespace Characteristic.
pub fn new() -> ServiceLabelNamespace {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::ServiceLabelNamespace,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead],
        valid_values: Some(vec![
            0, // "Dots"
            1, // "Arabic Numerals"
        ]),
        ..Default::default()
    })
}
