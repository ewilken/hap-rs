// THIS FILE IS AUTO-GENERATED

use crate::{
	accessory::{HapAccessory, HapAccessoryService, Accessory, Information},
	service::{HapService, accessory_information::AccessoryInformation, leak_sensor},
	event::EventEmitterPtr,
	Result,
};

/// Leak Sensor Accessory.
pub type LeakSensor = Accessory<LeakSensorInner>;

/// Inner type of the Leak Sensor Accessory.
#[derive(Default)]
pub struct LeakSensorInner {
    /// ID of the Leak Sensor Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Leak Sensor Service.
    pub leak_sensor: leak_sensor::LeakSensor,
}

impl HapAccessory for LeakSensorInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> {
        vec![
            &self.accessory_information,
            &self.leak_sensor,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.leak_sensor,
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

/// Creates a new Leak Sensor Accessory.
pub fn new(information: Information) -> Result<LeakSensor> {
    let mut leak_sensor = leak_sensor::new();
    leak_sensor.set_primary(true);
    Ok(LeakSensor::new(LeakSensorInner {
        accessory_information: information.to_service()?,
        leak_sensor,
        ..Default::default()
    }))
}
