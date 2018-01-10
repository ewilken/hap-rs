use characteristic;

pub type SerialNumber = characteristic::Characteristic<String>;

pub fn new() -> SerialNumber {
    SerialNumber {
        hap_type: "30".into(),
        perms: vec![
            characteristic::Perm::PairedRead
        ],
        value: "".into(),
        ..Default::default()
    }
}
