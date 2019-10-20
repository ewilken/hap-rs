// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Supported Video Stream Configuration Characteristic.
pub type SupportedVideoStreamConfiguration = Characteristic<Vec<u8>>;

/// Creates a new Supported Video Stream Configuration Characteristic.
pub fn new() -> SupportedVideoStreamConfiguration {
    Characteristic::new(Inner::<Vec<u8>> {
        hap_type: HapType::SupportedVideoStreamConfiguration,
        format: Format::Tlv8,
        perms: vec![Perm::PairedRead],
        ..Default::default()
    })
}
