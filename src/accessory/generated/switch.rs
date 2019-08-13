// THIS FILE IS AUTO-GENERATED

use crate::{
	accessory::{HapAccessory, HapAccessoryService, Accessory, Information},
	service::{HapService, accessory_information::AccessoryInformation, switch},
	event::EventEmitterPtr,
	Result,
};

/// Switch Accessory.
pub type Switch = Accessory<SwitchInner>;

/// Inner type of the Switch Accessory.
#[derive(Default)]
pub struct SwitchInner {
    /// ID of the Switch Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Switch Service.
    pub switch: switch::Switch,
}

impl HapAccessory for SwitchInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&HapAccessoryService> {
        vec![
            &self.accessory_information,
            &self.switch,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.switch,
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

/// Creates a new Switch Accessory.
pub fn new(information: Information) -> Result<Switch> {
    let mut switch = switch::new();
    switch.set_primary(true);
    Ok(Switch::new(SwitchInner {
        accessory_information: information.to_service()?,
        switch,
        ..Default::default()
    }))
}
