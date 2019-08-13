// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Audio Feedback Characteristic.
pub type AudioFeedback = Characteristic<bool>;

/// Creates a new Audio Feedback Characteristic.
pub fn new() -> AudioFeedback {
    Characteristic::new(Inner::<bool> {
        hap_type: HapType::AudioFeedback,
        format: Format::Bool,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
        ..Default::default()
    })
}
