// THIS FILE IS AUTO-GENERATED

use crate::{
	accessory::{HapAccessory, HapAccessoryService, Accessory, Information},
	service::{HapService, accessory_information::AccessoryInformation, window_covering},
	event::EventEmitterPtr,
	Result,
};

/// Window Covering Accessory.
pub type WindowCovering = Accessory<WindowCoveringInner>;

/// Inner type of the Window Covering Accessory.
#[derive(Default)]
pub struct WindowCoveringInner {
    /// ID of the Window Covering Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Window Covering Service.
    pub window_covering: window_covering::WindowCovering,
}

impl HapAccessory for WindowCoveringInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> {
        vec![
            &self.accessory_information,
            &self.window_covering,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.window_covering,
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

/// Creates a new Window Covering Accessory.
pub fn new(information: Information) -> Result<WindowCovering> {
    let mut window_covering = window_covering::new();
    window_covering.set_primary(true);
    Ok(WindowCovering::new(WindowCoveringInner {
        accessory_information: information.to_service()?,
        window_covering,
        ..Default::default()
    }))
}
