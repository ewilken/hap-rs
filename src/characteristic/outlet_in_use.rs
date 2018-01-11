use characteristic;

pub type OutletInUse = characteristic::Characteristic<bool>;

pub fn new() -> OutletInUse {
    OutletInUse {
        hap_type: "26".into(),
        perms: vec![
            characteristic::Perm::PairedRead,
            characteristic::Perm::Events,
        ],
        value: false,
        ..Default::default()
    }
}
