// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Obstruction Detected Characteristic.
pub type ObstructionDetected = Characteristic<bool>;

/// Creates a new Obstruction Detected Characteristic.
pub fn new() -> ObstructionDetected {
    Characteristic::new(Inner::<bool> {
        hap_type: HapType::ObstructionDetected,
        format: Format::Bool,
        perms: vec![Perm::PairedRead, Perm::Events],
        ..Default::default()
    })
}
