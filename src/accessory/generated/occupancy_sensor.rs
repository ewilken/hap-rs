// THIS FILE IS AUTO-GENERATED

use crate::{
	accessory::{HapAccessory, HapAccessoryService, Accessory, Information},
	service::{HapService, accessory_information::AccessoryInformation, occupancy_sensor},
	event::EventEmitterPtr,
	Result,
};

/// Occupancy Sensor Accessory.
pub type OccupancySensor = Accessory<OccupancySensorInner>;

/// Inner type of the Occupancy Sensor Accessory.
#[derive(Default)]
pub struct OccupancySensorInner {
    /// ID of the Occupancy Sensor Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Occupancy Sensor Service.
    pub occupancy_sensor: occupancy_sensor::OccupancySensor,
}

impl HapAccessory for OccupancySensorInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> {
        vec![
            &self.accessory_information,
            &self.occupancy_sensor,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.occupancy_sensor,
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

/// Creates a new Occupancy Sensor Accessory.
pub fn new(information: Information) -> Result<OccupancySensor> {
    let mut occupancy_sensor = occupancy_sensor::new();
    occupancy_sensor.set_primary(true);
    Ok(OccupancySensor::new(OccupancySensorInner {
        accessory_information: information.to_service()?,
        occupancy_sensor,
        ..Default::default()
    }))
}
