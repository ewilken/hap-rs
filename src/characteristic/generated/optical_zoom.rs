// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Optical Zoom Characteristic.
pub type OpticalZoom = Characteristic<f32>;

/// Creates a new Optical Zoom Characteristic.
pub fn new() -> OpticalZoom {
    Characteristic::new(Inner::<f32> {
        hap_type: HapType::OpticalZoom,
        format: Format::Float,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
        ..Default::default()
    })
}
