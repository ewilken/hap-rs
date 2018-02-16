use characteristic::{Characteristic, Format, Perm};

pub type FirmwareRevision = Characteristic<String>;

pub fn new() -> FirmwareRevision {
    FirmwareRevision {
        hap_type: "52".into(),
        format: Format::String,
        perms: vec![
            Perm::PairedRead,
        ],
        ..Default::default()
    }
}
