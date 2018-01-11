use characteristic;

pub type Identify = characteristic::Characteristic<bool>;

pub fn new() -> Identify {
    Identify {
        hap_type: "14".into(),
        perms: vec![
            characteristic::Perm::PairedWrite,
        ],
        value: false,
        ..Default::default()
    }
}
