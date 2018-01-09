use characteristic;

pub type On = characteristic::Characteristic<bool>;

pub fn new() -> On {
    On {
        hap_type: "25".into(),
        perms: vec![
            characteristic::Perm::PairedRead,
            characteristic::Perm::PairedWrite,
            characteristic::Perm::Events
        ],
        value: false,
        ..Default::default()
    }
}
