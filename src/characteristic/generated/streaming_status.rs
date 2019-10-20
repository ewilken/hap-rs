// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Streaming Status Characteristic.
pub type StreamingStatus = Characteristic<Vec<u8>>;

/// Creates a new Streaming Status Characteristic.
pub fn new() -> StreamingStatus {
    Characteristic::new(Inner::<Vec<u8>> {
        hap_type: HapType::StreamingStatus,
        format: Format::Tlv8,
        perms: vec![Perm::PairedRead, Perm::Events],
        ..Default::default()
    })
}
