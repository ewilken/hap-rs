// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Position State Characteristic.
pub type PositionState = Characteristic<u8>;

/// Creates a new Position State Characteristic.
pub fn new() -> PositionState {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::PositionState,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        valid_values: Some(vec![
            0, // "Decreasing"
            1, // "Increasing"
            2, // "Stopped"
        ]),
        ..Default::default()
    })
}
