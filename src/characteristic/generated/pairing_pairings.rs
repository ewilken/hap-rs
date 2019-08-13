// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Pairing Pairings Characteristic.
pub type PairingPairings = Characteristic<Vec<u8>>;

/// Creates a new Pairing Pairings Characteristic.
pub fn new() -> PairingPairings {
    Characteristic::new(Inner::<Vec<u8>> {
        hap_type: HapType::PairingPairings,
        format: Format::Tlv8,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
        ],
        ..Default::default()
    })
}
