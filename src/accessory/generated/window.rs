// THIS FILE IS AUTO-GENERATED

use crate::{
	accessory::{HapAccessory, HapAccessoryService, Accessory, Information},
	service::{HapService, accessory_information::AccessoryInformation, window},
	event::EventEmitterPtr,
	Result,
};

/// Window Accessory.
pub type Window = Accessory<WindowInner>;

/// Inner type of the Window Accessory.
#[derive(Default)]
pub struct WindowInner {
    /// ID of the Window Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Window Service.
    pub window: window::Window,
}

impl HapAccessory for WindowInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> {
        vec![
            &self.accessory_information,
            &self.window,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.window,
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

/// Creates a new Window Accessory.
pub fn new(information: Information) -> Result<Window> {
    let mut window = window::new();
    window.set_primary(true);
    Ok(Window::new(WindowInner {
        accessory_information: information.to_service()?,
        window,
        ..Default::default()
    }))
}
