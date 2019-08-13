// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Logs Characteristic.
pub type Logs = Characteristic<Vec<u8>>;

/// Creates a new Logs Characteristic.
pub fn new() -> Logs {
    Characteristic::new(Inner::<Vec<u8>> {
        hap_type: HapType::Logs,
        format: Format::Tlv8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
        ..Default::default()
    })
}
