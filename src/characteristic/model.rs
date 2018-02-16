use characteristic::{Characteristic, Format, Perm};

pub type Model = Characteristic<String>;

pub fn new() -> Model {
    Model {
        hap_type: "21".into(),
        format: Format::String,
        perms: vec![
            Perm::PairedRead,
        ],
        ..Default::default()
    }
}
