// THIS FILE IS AUTO-GENERATED

use crate::{
    accessory::{Accessory, HapAccessory, HapAccessoryService, Information},
    event::EventEmitterPtr,
    service::{accessory_information::AccessoryInformation, carbon_monoxide_sensor, HapService},
    Result,
};

/// Carbon Monoxide Sensor Accessory.
pub type CarbonMonoxideSensor = Accessory<CarbonMonoxideSensorInner>;

/// Inner type of the Carbon Monoxide Sensor Accessory.
#[derive(Default)]
pub struct CarbonMonoxideSensorInner {
    /// ID of the Carbon Monoxide Sensor Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Carbon Monoxide Sensor Service.
    pub carbon_monoxide_sensor: carbon_monoxide_sensor::CarbonMonoxideSensor,
}

impl HapAccessory for CarbonMonoxideSensorInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> {
        vec![&self.accessory_information, &self.carbon_monoxide_sensor]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![&mut self.accessory_information, &mut self.carbon_monoxide_sensor]
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

/// Creates a new Carbon Monoxide Sensor Accessory.
pub fn new(information: Information) -> Result<CarbonMonoxideSensor> {
    let mut carbon_monoxide_sensor = carbon_monoxide_sensor::new();
    carbon_monoxide_sensor.set_primary(true);
    Ok(CarbonMonoxideSensor::new(CarbonMonoxideSensorInner {
        accessory_information: information.to_service()?,
        carbon_monoxide_sensor,
        ..Default::default()
    }))
}
