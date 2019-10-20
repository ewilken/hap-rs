// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Supported RTP Configuration Characteristic.
pub type SupportedRTPConfiguration = Characteristic<Vec<u8>>;

/// Creates a new Supported RTP Configuration Characteristic.
pub fn new() -> SupportedRTPConfiguration {
    Characteristic::new(Inner::<Vec<u8>> {
        hap_type: HapType::SupportedRTPConfiguration,
        format: Format::Tlv8,
        perms: vec![Perm::PairedRead],
        ..Default::default()
    })
}
