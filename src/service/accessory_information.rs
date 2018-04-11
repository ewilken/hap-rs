use accessory::Information;
use service::{HapService, Service};
use characteristic::{
    HapCharacteristic,
    identify,
    manufacturer,
    model,
    name,
    serial_number,
    firmware_revision,
};
use hap_type::HapType;

pub type AccessoryInformation = Service<AccessoryInformationInner>;

impl Default for AccessoryInformation {
    fn default() -> AccessoryInformation { new(None) }
}

#[derive(Default)]
pub struct AccessoryInformationInner {
    id: u64,
    hap_type: HapType,
    hidden: bool,
    primary: bool,

    pub identify: identify::Identify,
    pub manufacturer: manufacturer::Manufacturer,
    pub model: model::Model,
    pub name: name::Name,
    pub serial_number: serial_number::SerialNumber,
    pub firmware_revision: firmware_revision::FirmwareRevision,
}

impl HapService for AccessoryInformationInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_type(&self) -> &HapType {
        &self.hap_type
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn get_primary(&self) -> bool {
        self.primary
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
}

pub fn new(information: Option<Information>) -> AccessoryInformation {
    AccessoryInformation::new({
        let mut inner = AccessoryInformationInner {
            hap_type: "3E".into(),
            identify: identify::new(),
            manufacturer: manufacturer::new(),
            model: model::new(),
            name: name::new(),
            serial_number: serial_number::new(),
            firmware_revision: firmware_revision::new(),
            ..Default::default()
        };
        if let Some(i) = information {
            inner.identify.set_value(i.identify).unwrap();
            inner.manufacturer.set_value(i.manufacturer).unwrap();
            inner.model.set_value(i.model).unwrap();
            inner.name.set_value(i.name).unwrap();
            inner.serial_number.set_value(i.serial_number).unwrap();
            inner.firmware_revision.set_value(i.firmware_revision).unwrap();
        }
        inner
    })
}
