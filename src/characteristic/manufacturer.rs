use characteristic::{Characteristic, Format, Perm};

pub type Manufacturer = Characteristic<String>;

pub fn new() -> Manufacturer {
    Manufacturer {
        hap_type: "20".into(),
        format: Format::String,
        perms: vec![
            Perm::PairedRead,
        ],
        ..Default::default()
    }
}
