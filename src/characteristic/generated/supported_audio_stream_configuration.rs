// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Supported Audio Stream Configuration Characteristic.
pub type SupportedAudioStreamConfiguration = Characteristic<Vec<u8>>;

/// Creates a new Supported Audio Stream Configuration Characteristic.
pub fn new() -> SupportedAudioStreamConfiguration {
    Characteristic::new(Inner::<Vec<u8>> {
        hap_type: HapType::SupportedAudioStreamConfiguration,
        format: Format::Tlv8,
        perms: vec![Perm::PairedRead],
        ..Default::default()
    })
}
