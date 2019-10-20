// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Motion Detected Characteristic.
pub type MotionDetected = Characteristic<bool>;

/// Creates a new Motion Detected Characteristic.
pub fn new() -> MotionDetected {
    Characteristic::new(Inner::<bool> {
        hap_type: HapType::MotionDetected,
        format: Format::Bool,
        perms: vec![Perm::PairedRead, Perm::Events],
        ..Default::default()
    })
}
