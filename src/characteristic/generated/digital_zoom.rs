// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Digital Zoom Characteristic.
pub type DigitalZoom = Characteristic<f32>;

/// Creates a new Digital Zoom Characteristic.
pub fn new() -> DigitalZoom {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::DigitalZoom,
        format: Format::Float,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        ..Default::default()
    })
}
