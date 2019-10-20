// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Pair Verify Characteristic.
pub type PairVerify = Characteristic<Vec<u8>>;

/// Creates a new Pair Verify Characteristic.
pub fn new() -> PairVerify {
    Characteristic::new(Inner::<Vec<u8>> {
        hap_type: HapType::PairVerify,
        format: Format::Tlv8,
        perms: vec![Perm::PairedRead, Perm::PairedWrite],
        ..Default::default()
    })
}
