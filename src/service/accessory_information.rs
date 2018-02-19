use serde_json;

use service::HapService;
use characteristic::{HapCharacteristic, identify, manufacturer, model, name, serial_number, firmware_revision};
use hap_type::HapType;

#[derive(Default)]
pub struct AccessoryInformation {
    id: u64,
    hap_type: HapType,

    pub identify: identify::Identify,
    pub manufacturer: manufacturer::Manufacturer,
    pub model: model::Model,
    pub name: name::Name,
    pub serial_number: serial_number::SerialNumber,
    pub firmware_revision: firmware_revision::FirmwareRevision,
}

impl HapService for AccessoryInformation {
    fn get_id(&self) -> &u64 {
        &self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_type(&self) -> &HapType {
        &self.hap_type
    }

    fn get_characteristics(&self) -> Vec<&HapCharacteristic> {
        vec![
            &self.identify,
            &self.manufacturer,
            &self.model,
            &self.name,
            &self.serial_number,
            &self.firmware_revision,
        ]
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> {
        vec![
            &mut self.identify,
            &mut self.manufacturer,
            &mut self.model,
            &mut self.name,
            &mut self.serial_number,
            &mut self.firmware_revision,
        ]
    }

    fn to_json(&self) -> serde_json::Value {
        let characteristics: Vec<serde_json::Value> = self.get_characteristics().iter().map(|c| c.to_json()).collect();
        json!({
            "type": self.get_type(),
            "iid": self.get_id(),
            "characteristics": characteristics,
        })
    }
}

pub fn new() -> AccessoryInformation {
    AccessoryInformation {
        hap_type: "3E".into(),
        identify: identify::new(),
        manufacturer: manufacturer::new(),
        model: model::new(),
        name: name::new(),
        serial_number: serial_number::new(),
        firmware_revision: firmware_revision::new(),
        ..Default::default()
    }
}
