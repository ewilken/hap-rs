use characteristic;
use service::ServiceT;
use hap_type;

#[derive(Default)]
pub struct AccessoryInformation {
    id: u64,
    hap_type: hap_type::HapType,

    pub identify: characteristic::identify::Identify,
    pub manufacturer: characteristic::manufacturer::Manufacturer,
    pub model: characteristic::model::Model,
    pub name: characteristic::name::Name,
    pub serial_number: characteristic::serial_number::SerialNumber,
    pub firmware_revision: characteristic::firmware_revision::FirmwareRevision,
}

impl ServiceT for AccessoryInformation {
    fn get_characteristics(&self) -> Vec<&characteristic::CharacteristicT> {
        vec![
            &self.identify,
            &self.manufacturer,
            &self.model,
            &self.name,
            &self.serial_number,
            &self.firmware_revision,
        ]
    }
}

pub fn new() -> AccessoryInformation {
    AccessoryInformation {
        hap_type: "3E".into(),
        identify: characteristic::identify::new(),
        manufacturer: characteristic::manufacturer::new(),
        model: characteristic::model::new(),
        name: characteristic::name::new(),
        serial_number: characteristic::serial_number::new(),
        firmware_revision: characteristic::firmware_revision::new(),
        ..Default::default()
    }
}
