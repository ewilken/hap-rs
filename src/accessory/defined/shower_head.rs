use futures::executor;
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    accessory::{AccessoryInformation, HapAccessory},
    characteristic::HapCharacteristic,
    service::{accessory_information::AccessoryInformationService, valve::ValveService, HapService},
    HapType,
    Result,
};

/// Shower Head Accessory.
#[derive(Debug, Default)]
pub struct ShowerHeadAccessory {
    /// ID of the Shower Head Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformationService,
    /// Valve Service.
    pub valve: ValveService,
}

impl ShowerHeadAccessory {
    /// Creates a new Shower Head Accessory.
    pub fn new(id: u64, information: AccessoryInformation) -> Result<Self> {
        let accessory_information = information.to_service(1, id)?;

        let valve_id = 2 + accessory_information.get_characteristics().len() as u64;
        let mut valve = ValveService::new(valve_id, id);
        valve.set_primary(true);
        executor::block_on(valve.valve_type.set_value(2.into()))?; // 2 is SHOWER_HEAD

        Ok(Self {
            id,
            accessory_information,
            valve,
        })
    }
}

impl HapAccessory for ShowerHeadAccessory {
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

    fn get_services(&self) -> Vec<&dyn HapService> { vec![&self.accessory_information, &self.valve] }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
        vec![&mut self.accessory_information, &mut self.valve]
    }
}

impl Serialize for ShowerHeadAccessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}
