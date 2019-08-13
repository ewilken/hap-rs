// THIS FILE IS AUTO-GENERATED

use crate::{
	accessory::{HapAccessory, HapAccessoryService, Accessory, Information},
	service::{HapService, accessory_information::AccessoryInformation, garage_door_opener},
	event::EventEmitterPtr,
	Result,
};

/// Garage Door Opener Accessory.
pub type GarageDoorOpener = Accessory<GarageDoorOpenerInner>;

/// Inner type of the Garage Door Opener Accessory.
#[derive(Default)]
pub struct GarageDoorOpenerInner {
    /// ID of the Garage Door Opener Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Garage Door Opener Service.
    pub garage_door_opener: garage_door_opener::GarageDoorOpener,
}

impl HapAccessory for GarageDoorOpenerInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&HapAccessoryService> {
        vec![
            &self.accessory_information,
            &self.garage_door_opener,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.garage_door_opener,
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

/// Creates a new Garage Door Opener Accessory.
pub fn new(information: Information) -> Result<GarageDoorOpener> {
    let mut garage_door_opener = garage_door_opener::new();
    garage_door_opener.set_primary(true);
    Ok(GarageDoorOpener::new(GarageDoorOpenerInner {
        accessory_information: information.to_service()?,
        garage_door_opener,
        ..Default::default()
    }))
}
