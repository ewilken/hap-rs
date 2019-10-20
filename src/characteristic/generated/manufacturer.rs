// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Manufacturer Characteristic.
pub type Manufacturer = Characteristic<String>;

/// Creates a new Manufacturer Characteristic.
pub fn new() -> Manufacturer {
    Characteristic::new(Inner::<String> {
        hap_type: HapType::Manufacturer,
        format: Format::String,
        perms: vec![Perm::PairedRead],
        ..Default::default()
    })
}
