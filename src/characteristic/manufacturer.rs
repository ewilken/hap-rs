use characteristic;

pub type Manufacturer = characteristic::Characteristic<String>;

pub fn new() -> Manufacturer {
    Manufacturer {
        hap_type: "20".into(),
        perms: vec![
            characteristic::Perm::PairedRead
        ],
        value: "".into(),
        ..Default::default()
    }
}
