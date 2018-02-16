use characteristic::{Characteristic, Format, Perm};

pub type Name = Characteristic<String>;

pub fn new() -> Name {
    Name {
        hap_type: "23".into(),
        format: Format::String,
        perms: vec![
            Perm::PairedRead,
        ],
        ..Default::default()
    }
}
