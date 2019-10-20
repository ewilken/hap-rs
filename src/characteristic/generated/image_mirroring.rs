// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Image Mirroring Characteristic.
pub type ImageMirroring = Characteristic<bool>;

/// Creates a new Image Mirroring Characteristic.
pub fn new() -> ImageMirroring {
    Characteristic::new(Inner::<bool> {
        hap_type: HapType::ImageMirroring,
        format: Format::Bool,
        perms: vec![Perm::PairedRead, Perm::PairedWrite, Perm::Events],
        ..Default::default()
    })
}
