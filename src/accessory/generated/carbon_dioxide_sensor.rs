// THIS FILE IS AUTO-GENERATED

use crate::{
    accessory::{Accessory, HapAccessory, HapAccessoryService, Information},
    event::EventEmitterPtr,
    service::{accessory_information::AccessoryInformation, carbon_dioxide_sensor, HapService},
    Result,
};

/// Carbon Dioxide Sensor Accessory.
pub type CarbonDioxideSensor = Accessory<CarbonDioxideSensorInner>;

/// Inner type of the Carbon Dioxide Sensor Accessory.
#[derive(Default)]
pub struct CarbonDioxideSensorInner {
    /// ID of the Carbon Dioxide Sensor Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Carbon Dioxide Sensor Service.
    pub carbon_dioxide_sensor: carbon_dioxide_sensor::CarbonDioxideSensor,
}

impl HapAccessory for CarbonDioxideSensorInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> {
        vec![&self.accessory_information, &self.carbon_dioxide_sensor]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![&mut self.accessory_information, &mut self.carbon_dioxide_sensor]
    }

    fn get_mut_information(&mut self) -> &mut AccessoryInformation { &mut self.accessory_information }

    fn init_iids(&mut self, accessory_id: u64, event_emitter: EventEmitterPtr) -> Result<()> {
        let mut next_iid = 1;
        for service in self.get_mut_services() {
            service.set_id(next_iid);
            next_iid += 1;
            for characteristic in service.get_mut_characteristics() {
                characteristic.set_id(next_iid)?;
                characteristic.set_accessory_id(accessory_id)?;
                characteristic.set_event_emitter(Some(event_emitter.clone()))?;
                next_iid += 1;
            }
        }
        Ok(())
    }
}

/// Creates a new Carbon Dioxide Sensor Accessory.
pub fn new(information: Information) -> Result<CarbonDioxideSensor> {
    let mut carbon_dioxide_sensor = carbon_dioxide_sensor::new();
    carbon_dioxide_sensor.set_primary(true);
    Ok(CarbonDioxideSensor::new(CarbonDioxideSensorInner {
        accessory_information: information.to_service()?,
        carbon_dioxide_sensor,
        ..Default::default()
    }))
}
