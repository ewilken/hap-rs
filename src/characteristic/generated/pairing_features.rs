// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Pairing Features Characteristic.
pub type PairingFeatures = Characteristic<u8>;

/// Creates a new Pairing Features Characteristic.
pub fn new() -> PairingFeatures {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::PairingFeatures,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
        ],
        ..Default::default()
    })
}
