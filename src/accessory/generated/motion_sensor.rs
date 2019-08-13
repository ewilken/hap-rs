// THIS FILE IS AUTO-GENERATED

use crate::{
	accessory::{HapAccessory, HapAccessoryService, Accessory, Information},
	service::{HapService, accessory_information::AccessoryInformation, motion_sensor},
	event::EventEmitterPtr,
	Result,
};

/// Motion Sensor Accessory.
pub type MotionSensor = Accessory<MotionSensorInner>;

/// Inner type of the Motion Sensor Accessory.
#[derive(Default)]
pub struct MotionSensorInner {
    /// ID of the Motion Sensor Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Motion Sensor Service.
    pub motion_sensor: motion_sensor::MotionSensor,
}

impl HapAccessory for MotionSensorInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&HapAccessoryService> {
        vec![
            &self.accessory_information,
            &self.motion_sensor,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.motion_sensor,
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

/// Creates a new Motion Sensor Accessory.
pub fn new(information: Information) -> Result<MotionSensor> {
    let mut motion_sensor = motion_sensor::new();
    motion_sensor.set_primary(true);
    Ok(MotionSensor::new(MotionSensorInner {
        accessory_information: information.to_service()?,
        motion_sensor,
        ..Default::default()
    }))
}
