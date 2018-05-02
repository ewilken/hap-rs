use characteristic::{Characteristic, Format, Perm};

pub type OutletInUse = Characteristic<bool>;

pub fn new() -> OutletInUse {
    OutletInUse {
        hap_type: "26".into(),
        format: Format::Bool,
        perms: vec![
            Perm::PairedRead,
            Perm::Events,
        ],
        value: true,
        ..Default::default()
    }
}
