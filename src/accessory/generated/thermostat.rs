// THIS FILE IS AUTO-GENERATED

use crate::{
	accessory::{HapAccessory, HapAccessoryService, Accessory, Information},
	service::{HapService, accessory_information::AccessoryInformation, thermostat},
	event::EventEmitterPtr,
	Result,
};

/// Thermostat Accessory.
pub type Thermostat = Accessory<ThermostatInner>;

/// Inner type of the Thermostat Accessory.
#[derive(Default)]
pub struct ThermostatInner {
    /// ID of the Thermostat Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Thermostat Service.
    pub thermostat: thermostat::Thermostat,
}

impl HapAccessory for ThermostatInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> {
        vec![
            &self.accessory_information,
            &self.thermostat,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.thermostat,
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

/// Creates a new Thermostat Accessory.
pub fn new(information: Information) -> Result<Thermostat> {
    let mut thermostat = thermostat::new();
    thermostat.set_primary(true);
    Ok(Thermostat::new(ThermostatInner {
        accessory_information: information.to_service()?,
        thermostat,
        ..Default::default()
    }))
}
