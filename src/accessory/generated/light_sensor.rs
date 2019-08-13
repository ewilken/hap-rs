// THIS FILE IS AUTO-GENERATED

use crate::{
	accessory::{HapAccessory, HapAccessoryService, Accessory, Information},
	service::{HapService, accessory_information::AccessoryInformation, light_sensor},
	event::EventEmitterPtr,
	Result,
};

/// Light Sensor Accessory.
pub type LightSensor = Accessory<LightSensorInner>;

/// Inner type of the Light Sensor Accessory.
#[derive(Default)]
pub struct LightSensorInner {
    /// ID of the Light Sensor Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Light Sensor Service.
    pub light_sensor: light_sensor::LightSensor,
}

impl HapAccessory for LightSensorInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&HapAccessoryService> {
        vec![
            &self.accessory_information,
            &self.light_sensor,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.light_sensor,
        ]
    }

    fn get_mut_information(&mut self) -> &mut AccessoryInformation {
        &mut self.accessory_information
    }

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

/// Creates a new Light Sensor Accessory.
pub fn new(information: Information) -> Result<LightSensor> {
    let mut light_sensor = light_sensor::new();
    light_sensor.set_primary(true);
    Ok(LightSensor::new(LightSensorInner {
        accessory_information: information.to_service()?,
        light_sensor,
        ..Default::default()
    }))
}
