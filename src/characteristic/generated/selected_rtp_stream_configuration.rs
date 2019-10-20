// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Selected RTP Stream Configuration Characteristic.
pub type SelectedRTPStreamConfiguration = Characteristic<Vec<u8>>;

/// Creates a new Selected RTP Stream Configuration Characteristic.
pub fn new() -> SelectedRTPStreamConfiguration {
    Characteristic::new(Inner::<Vec<u8>> {
        hap_type: HapType::SelectedRTPStreamConfiguration,
        format: Format::Tlv8,
        perms: vec![Perm::PairedRead, Perm::PairedWrite],
        ..Default::default()
    })
}
