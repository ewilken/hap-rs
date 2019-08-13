// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Pair Setup Characteristic.
pub type PairSetup = Characteristic<Vec<u8>>;

/// Creates a new Pair Setup Characteristic.
pub fn new() -> PairSetup {
    Characteristic::new(Inner::<Vec<u8>> {
        hap_type: HapType::PairSetup,
        format: Format::Tlv8,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
        ],
        ..Default::default()
    })
}
