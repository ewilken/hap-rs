// THIS FILE IS AUTO-GENERATED

use crate::{
    accessory::{Accessory, HapAccessory, HapAccessoryService, Information},
    event::EventEmitterPtr,
    service::{accessory_information::AccessoryInformation, valve, HapService},
    Result,
};

/// Valve Accessory.
pub type Valve = Accessory<ValveInner>;

/// Inner type of the Valve Accessory.
#[derive(Default)]
pub struct ValveInner {
    /// ID of the Valve Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Valve Service.
    pub valve: valve::Valve,
}

impl HapAccessory for ValveInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> { vec![&self.accessory_information, &self.valve] }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![&mut self.accessory_information, &mut self.valve]
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

/// Creates a new Valve Accessory.
pub fn new(information: Information) -> Result<Valve> {
    let mut valve = valve::new();
    valve.set_primary(true);
    Ok(Valve::new(ValveInner {
        accessory_information: information.to_service()?,
        valve,
        ..Default::default()
    }))
}
