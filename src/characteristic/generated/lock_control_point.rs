// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Lock Control Point Characteristic.
pub type LockControlPoint = Characteristic<Vec<u8>>;

/// Creates a new Lock Control Point Characteristic.
pub fn new() -> LockControlPoint {
    Characteristic::new(Inner::<Vec<u8>> {
        hap_type: HapType::LockControlPoint,
        format: Format::Tlv8,
        perms: vec![Perm::PairedWrite],
        ..Default::default()
    })
}
