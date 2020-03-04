use crate::{
    accessory::{Accessory, HapAccessory, HapAccessoryService, Information},
    pointer,
    service::{accessory_information::AccessoryInformation, speaker, television, HapService},
    Result,
};

/// Television Accessory.
pub type Television = Accessory<TelevisionInner>;

/// Inner type of the Television Accessory.
#[derive(Default)]
pub struct TelevisionInner {
    /// ID of the Television Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Television Service.
    pub television: television::Television,
    /// Speaker Service.
    pub speaker: speaker::Speaker,
}

impl HapAccessory for TelevisionInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> {
        vec![&self.accessory_information, &self.television, &self.speaker]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![&mut self.accessory_information, &mut self.television, &mut self.speaker]
    }

    fn get_mut_information(&mut self) -> &mut AccessoryInformation { &mut self.accessory_information }

    fn init_iids(&mut self, accessory_id: u64, event_emitter: pointer::EventEmitter) -> Result<()> {
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

/// Creates a new Television Accessory.
pub fn new(information: Information) -> Result<Television> {
    let mut television = television::new();
    television.set_primary(true);
    Ok(Television::new(TelevisionInner {
        accessory_information: information.to_service()?,
        television,
        speaker: speaker::new(),
        ..Default::default()
    }))
}
