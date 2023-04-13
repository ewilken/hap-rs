use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    accessory::{AccessoryInformation, HapAccessory},
    service::{accessory_information::AccessoryInformationService, heater_cooler::HeaterCoolerService, HapService},
    HapType,
    Result,
};

/// Heater-Cooler Accessory.
#[derive(Debug, Default)]
pub struct HeaterCoolerAccessory {
    /// ID of the Heater-Cooler Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformationService,
    /// Heater-Cooler Service.
    pub heater_cooler: HeaterCoolerService,
}

impl HeaterCoolerAccessory {
    /// Creates a new Heater-Cooler Accessory.
    pub fn new(id: u64, information: AccessoryInformation) -> Result<Self> {
        let accessory_information = information.to_service(1, id)?;
        let heater_cooler_id = accessory_information.get_characteristics().len() as u64;
        let mut heater_cooler = HeaterCoolerService::new(1 + heater_cooler_id + 1, id);
        heater_cooler.set_primary(true);

        Ok(Self {
            id,
            accessory_information,
            heater_cooler,
        })
    }
}

impl HapAccessory for HeaterCoolerAccessory {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_service(&self, hap_type: HapType) -> Option<&dyn HapService> {
        for service in self.get_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_mut_service(&mut self, hap_type: HapType) -> Option<&mut dyn HapService> {
        for service in self.get_mut_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_services(&self) -> Vec<&dyn HapService> { vec![&self.accessory_information, &self.heater_cooler] }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
        vec![&mut self.accessory_information, &mut self.heater_cooler]
    }
}

impl Serialize for HeaterCoolerAccessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}
