use service::Service;
use characteristic::{identify, manufacturer, model, name, serial_number, firmware_revision};

pub type AccessoryInformation = Service;

pub fn new() -> AccessoryInformation {
    AccessoryInformation {
        hap_type: "3E".into(),
        characteristics: vec![
            Box::new(identify::new()),
            Box::new(manufacturer::new()),
            Box::new(model::new()),
            Box::new(name::new()),
            Box::new(serial_number::new()),
            Box::new(firmware_revision::new()),
        ],
        ..Default::default()
    }
}
