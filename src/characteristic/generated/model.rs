// THIS FILE IS AUTO-GENERATED

use crate::characteristic::{Characteristic, Format, HapType, Inner, Perm};

/// Model Characteristic.
pub type Model = Characteristic<String>;

/// Creates a new Model Characteristic.
pub fn new() -> Model {
    Characteristic::new(Inner::<String> {
        hap_type: HapType::Model,
        format: Format::String,
        perms: vec![Perm::PairedRead],
        ..Default::default()
    })
}
