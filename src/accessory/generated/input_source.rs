// THIS FILE IS AUTO-GENERATED

use crate::{
	accessory::{HapAccessory, HapAccessoryService, Accessory, Information},
	service::{HapService, accessory_information::AccessoryInformation, input_source},
	event::EventEmitterPtr,
	Result,
};

/// Input Source Accessory.
pub type InputSource = Accessory<InputSourceInner>;

/// Inner type of the Input Source Accessory.
#[derive(Default)]
pub struct InputSourceInner {
    /// ID of the Input Source Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Input Source Service.
    pub input_source: input_source::InputSource,
}

impl HapAccessory for InputSourceInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&HapAccessoryService> {
        vec![
            &self.accessory_information,
            &self.input_source,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.input_source,
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

/// Creates a new Input Source Accessory.
pub fn new(information: Information) -> Result<InputSource> {
    let mut input_source = input_source::new();
    input_source.set_primary(true);
    Ok(InputSource::new(InputSourceInner {
        accessory_information: information.to_service()?,
        input_source,
        ..Default::default()
    }))
}
