use characteristic::{Characteristic, Format, Perm};

pub type On = Characteristic<bool>;

pub fn new() -> On {
    On {
        hap_type: "25".into(),
        format: Format::Bool,
        perms: vec![
            Perm::PairedRead,
            Perm::PairedWrite,
            Perm::Events,
        ],
        value: Some(false),
        ..Default::default()
    }
}
