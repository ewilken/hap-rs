// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Digital Zoom Characteristic.
pub type DigitalZoom = Characteristic<f32>;

/// Creates a new Digital Zoom Characteristic.
pub fn new() -> DigitalZoom {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::DigitalZoom,
        format: Format::Float,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
        ..Default::default()
    })
}
