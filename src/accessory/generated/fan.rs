// THIS FILE IS AUTO-GENERATED

use crate::{
    accessory::{Accessory, HapAccessory, HapAccessoryService, Information},
    event::EventEmitterPtr,
    service::{accessory_information::AccessoryInformation, fan, HapService},
    Result,
};

/// Fan Accessory.
pub type Fan = Accessory<FanInner>;

/// Inner type of the Fan Accessory.
#[derive(Default)]
pub struct FanInner {
    /// ID of the Fan Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Fan Service.
    pub fan: fan::Fan,
}

impl HapAccessory for FanInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> { vec![&self.accessory_information, &self.fan] }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![&mut self.accessory_information, &mut self.fan]
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

/// Creates a new Fan Accessory.
pub fn new(information: Information) -> Result<Fan> {
    let mut fan = fan::new();
    fan.set_primary(true);
    Ok(Fan::new(FanInner {
        accessory_information: information.to_service()?,
        fan,
        ..Default::default()
    }))
}
