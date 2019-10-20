// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Input Source Type Characteristic.
pub type InputSourceType = Characteristic<u8>;

/// Creates a new Input Source Type Characteristic.
pub fn new() -> InputSourceType {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::InputSourceType,
        format: Format::UInt8,
        perms: vec![Perm::PairedRead, Perm::Events],
        max_value: Some(10),
        min_value: Some(0),
        step_value: Some(1),
        valid_values: Some(vec![
            0,  // "Other"
            1,  // "HomeScreen"
            10, // "Application"
            2,  // "Tuner"
            3,  // "Hdmi"
            4,  // "CompositeVideo"
            5,  // "SVideo"
            6,  // "ComponentVideo"
            7,  // "Dvi"
            8,  // "Airplay"
            9,  // "Usb"
        ]),
        ..Default::default()
    })
}
