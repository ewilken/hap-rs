// THIS FILE IS AUTO-GENERATED

use crate::{
    accessory::{Accessory, HapAccessory, HapAccessoryService, Information},
    event::EventEmitterPtr,
    service::{accessory_information::AccessoryInformation, air_purifier, HapService},
    Result,
};

/// Air Purifier Accessory.
pub type AirPurifier = Accessory<AirPurifierInner>;

/// Inner type of the Air Purifier Accessory.
#[derive(Default)]
pub struct AirPurifierInner {
    /// ID of the Air Purifier Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Air Purifier Service.
    pub air_purifier: air_purifier::AirPurifier,
}

impl HapAccessory for AirPurifierInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> { vec![&self.accessory_information, &self.air_purifier] }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![&mut self.accessory_information, &mut self.air_purifier]
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

/// Creates a new Air Purifier Accessory.
pub fn new(information: Information) -> Result<AirPurifier> {
    let mut air_purifier = air_purifier::new();
    air_purifier.set_primary(true);
    Ok(AirPurifier::new(AirPurifierInner {
        accessory_information: information.to_service()?,
        air_purifier,
        ..Default::default()
    }))
}
