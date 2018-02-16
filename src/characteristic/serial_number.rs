use characteristic::{Characteristic, Format, Perm};

pub type SerialNumber = Characteristic<String>;

pub fn new() -> SerialNumber {
    SerialNumber {
        hap_type: "30".into(),
        format: Format::String,
        perms: vec![
            Perm::PairedRead,
        ],
        ..Default::default()
    }
}
