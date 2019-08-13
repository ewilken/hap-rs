// THIS FILE IS AUTO-GENERATED

use crate::{
	accessory::{HapAccessory, HapAccessoryService, Accessory, Information},
	service::{HapService, accessory_information::AccessoryInformation, stateless_programmable_switch},
	event::EventEmitterPtr,
	Result,
};

/// Stateless Programmable Switch Accessory.
pub type StatelessProgrammableSwitch = Accessory<StatelessProgrammableSwitchInner>;

/// Inner type of the Stateless Programmable Switch Accessory.
#[derive(Default)]
pub struct StatelessProgrammableSwitchInner {
    /// ID of the Stateless Programmable Switch Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Stateless Programmable Switch Service.
    pub stateless_programmable_switch: stateless_programmable_switch::StatelessProgrammableSwitch,
}

impl HapAccessory for StatelessProgrammableSwitchInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&HapAccessoryService> {
        vec![
            &self.accessory_information,
            &self.stateless_programmable_switch,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.stateless_programmable_switch,
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

/// Creates a new Stateless Programmable Switch Accessory.
pub fn new(information: Information) -> Result<StatelessProgrammableSwitch> {
    let mut stateless_programmable_switch = stateless_programmable_switch::new();
    stateless_programmable_switch.set_primary(true);
    Ok(StatelessProgrammableSwitch::new(StatelessProgrammableSwitchInner {
        accessory_information: information.to_service()?,
        stateless_programmable_switch,
        ..Default::default()
    }))
}
