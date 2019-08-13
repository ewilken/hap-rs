// THIS FILE IS AUTO-GENERATED

use crate::{
	accessory::{HapAccessory, HapAccessoryService, Accessory, Information},
	service::{HapService, accessory_information::AccessoryInformation, fan_v2},
	event::EventEmitterPtr,
	Result,
};

/// Fan v2 Accessory.
pub type Fanv2 = Accessory<Fanv2Inner>;

/// Inner type of the Fan v2 Accessory.
#[derive(Default)]
pub struct Fanv2Inner {
    /// ID of the Fan v2 Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Fan v2 Service.
    pub fan_v2: fan_v2::Fanv2,
}

impl HapAccessory for Fanv2Inner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> {
        vec![
            &self.accessory_information,
            &self.fan_v2,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.fan_v2,
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

/// Creates a new Fan v2 Accessory.
pub fn new(information: Information) -> Result<Fanv2> {
    let mut fan_v2 = fan_v2::new();
    fan_v2.set_primary(true);
    Ok(Fanv2::new(Fanv2Inner {
        accessory_information: information.to_service()?,
        fan_v2,
        ..Default::default()
    }))
}
