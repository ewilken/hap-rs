// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Setup Endpoints Characteristic.
pub type SetupEndpoints = Characteristic<Vec<u8>>;

/// Creates a new Setup Endpoints Characteristic.
pub fn new() -> SetupEndpoints {
    Characteristic::new(Inner::<Vec<u8>> {
        hap_type: HapType::SetupEndpoints,
        format: Format::Tlv8,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
        ],
        ..Default::default()
    })
}
