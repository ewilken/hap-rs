use characteristic;

pub type Name = characteristic::Characteristic<String>;

pub fn new() -> Name {
    Name {
        hap_type: "23".into(),
        perms: vec![
            characteristic::Perm::PairedRead,
        ],
        value: "".into(),
        ..Default::default()
    }
}
