// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{HapType, Characteristic, Inner, Format, Perm, Unit};

/// Lock Management Auto Security Timeout Characteristic.
pub type LockManagementAutoSecurityTimeout = Characteristic<u32>;

/// Creates a new Lock Management Auto Security Timeout Characteristic.
pub fn new() -> LockManagementAutoSecurityTimeout {
    Characteristic::new(Inner::<u32> {
        hap_type: HapType::LockManagementAutoSecurityTimeout,
        format: Format::UInt32,
        perms: vec![
			Perm::PairedRead,
			Perm::PairedWrite,
			Perm::Events,
        ],
		unit: Some(Unit::Seconds),
        ..Default::default()
    })
}
