// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Display Order Characteristic.
pub type DisplayOrder = Characteristic<Vec<u8>>;

/// Creates a new Display Order Characteristic.
pub fn new() -> DisplayOrder {
    Characteristic::new(Inner::<Vec<u8>> {
        hap_type: HapType::DisplayOrder,
        format: Format::Tlv8,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        ..Default::default()
    })
}
