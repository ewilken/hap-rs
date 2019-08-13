// THIS FILE IS AUTO-GENERATED

use crate::{
	accessory::{HapAccessory, HapAccessoryService, Accessory, Information},
	service::{HapService, accessory_information::AccessoryInformation, lightbulb},
	event::EventEmitterPtr,
	Result,
};

/// Lightbulb Accessory.
pub type Lightbulb = Accessory<LightbulbInner>;

/// Inner type of the Lightbulb Accessory.
#[derive(Default)]
pub struct LightbulbInner {
    /// ID of the Lightbulb Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Lightbulb Service.
    pub lightbulb: lightbulb::Lightbulb,
}

impl HapAccessory for LightbulbInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> {
        vec![
            &self.accessory_information,
            &self.lightbulb,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.lightbulb,
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

/// Creates a new Lightbulb Accessory.
pub fn new(information: Information) -> Result<Lightbulb> {
    let mut lightbulb = lightbulb::new();
    lightbulb.set_primary(true);
    Ok(Lightbulb::new(LightbulbInner {
        accessory_information: information.to_service()?,
        lightbulb,
        ..Default::default()
    }))
}
