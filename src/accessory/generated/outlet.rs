// THIS FILE IS AUTO-GENERATED

use crate::{
    accessory::{Accessory, HapAccessory, HapAccessoryService, Information},
    event::EventEmitterPtr,
    service::{accessory_information::AccessoryInformation, outlet, HapService},
    Result,
};

/// Outlet Accessory.
pub type Outlet = Accessory<OutletInner>;

/// Inner type of the Outlet Accessory.
#[derive(Default)]
pub struct OutletInner {
    /// ID of the Outlet Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Outlet Service.
    pub outlet: outlet::Outlet,
}

impl HapAccessory for OutletInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> { vec![&self.accessory_information, &self.outlet] }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![&mut self.accessory_information, &mut self.outlet]
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

/// Creates a new Outlet Accessory.
pub fn new(information: Information) -> Result<Outlet> {
    let mut outlet = outlet::new();
    outlet.set_primary(true);
    Ok(Outlet::new(OutletInner {
        accessory_information: information.to_service()?,
        outlet,
        ..Default::default()
    }))
}
