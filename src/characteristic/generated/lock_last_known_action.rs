// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm};

/// Lock Last Known Action Characteristic.
pub type LockLastKnownAction = Characteristic<u8>;

/// Creates a new Lock Last Known Action Characteristic.
pub fn new() -> LockLastKnownAction {
    Characteristic::new(Inner::<u8> {
        hap_type: HapType::LockLastKnownAction,
        format: Format::UInt8,
        perms: vec![
			Perm::PairedRead,
			Perm::Events,
        ],
		valid_values: Some(vec![
			0, // "Secured Physically, Interior"
			1, // "Unsecured Physically, Interior"
			2, // "Secured Physically, Exterior"
			3, // "Unsecured Physically, Exterior"
			4, // "Secured by Keypad"
			5, // "Unsecured by Keypad"
			6, // "Secured Remotely"
			7, // "Unsecured Remotely"
			8, // "Secured by Auto Secure Timeout"
		]),
        ..Default::default()
    })
}
