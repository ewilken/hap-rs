use characteristic;

pub type FirmwareRevision = characteristic::Characteristic<String>;

pub fn new() -> FirmwareRevision {
    FirmwareRevision {
        hap_type: "52".into(),
        perms: vec![
            characteristic::Perm::PairedRead,
        ],
        value: "".into(),
        ..Default::default()
    }
}
