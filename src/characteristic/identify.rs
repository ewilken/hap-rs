use characteristic::{Characteristic, Format, Perm};

pub type Identify = Characteristic<bool>;

pub fn new() -> Identify {
    Identify {
        hap_type: "14".into(),
        format: Format::Bool,
        perms: vec![
            Perm::PairedWrite,
        ],
        ..Default::default()
    }
}
