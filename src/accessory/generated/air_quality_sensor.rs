// THIS FILE IS AUTO-GENERATED

use crate::{
    accessory::{Accessory, HapAccessory, HapAccessoryService, Information},
    event::EventEmitterPtr,
    service::{accessory_information::AccessoryInformation, air_quality_sensor, HapService},
    Result,
};

/// Air Quality Sensor Accessory.
pub type AirQualitySensor = Accessory<AirQualitySensorInner>;

/// Inner type of the Air Quality Sensor Accessory.
#[derive(Default)]
pub struct AirQualitySensorInner {
    /// ID of the Air Quality Sensor Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Air Quality Sensor Service.
    pub air_quality_sensor: air_quality_sensor::AirQualitySensor,
}

impl HapAccessory for AirQualitySensorInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> {
        vec![&self.accessory_information, &self.air_quality_sensor]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![&mut self.accessory_information, &mut self.air_quality_sensor]
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

/// Creates a new Air Quality Sensor Accessory.
pub fn new(information: Information) -> Result<AirQualitySensor> {
    let mut air_quality_sensor = air_quality_sensor::new();
    air_quality_sensor.set_primary(true);
    Ok(AirQualitySensor::new(AirQualitySensorInner {
        accessory_information: information.to_service()?,
        air_quality_sensor,
        ..Default::default()
    }))
}
