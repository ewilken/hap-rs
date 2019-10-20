// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Programmable Switch Event Characteristic.
pub type ProgrammableSwitchEvent = Characteristic<u8>;

/// Creates a new Programmable Switch Event Characteristic.
pub fn new() -> ProgrammableSwitchEvent {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::ProgrammableSwitchEvent,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        valid_values: Some(vec![
            0, // "Single Press"
            1, // "Double Press"
            2, // "Long Press"
        ]),
        ..Default::default()
    })
}
