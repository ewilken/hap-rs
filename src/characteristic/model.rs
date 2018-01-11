use characteristic;

pub type Model = characteristic::Characteristic<String>;

pub fn new() -> Model {
    Model {
        hap_type: "21".into(),
        perms: vec![
            characteristic::Perm::PairedRead,
        ],
        value: "".into(),
        ..Default::default()
    }
}
