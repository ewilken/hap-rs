// THIS FILE IS AUTO-GENERATED

use crate::{
	accessory::{HapAccessory, HapAccessoryService, Accessory, Information},
	service::{HapService, accessory_information::AccessoryInformation, humidifier_dehumidifier},
	event::EventEmitterPtr,
	Result,
};

/// Humidifier Dehumidifier Accessory.
pub type HumidifierDehumidifier = Accessory<HumidifierDehumidifierInner>;

/// Inner type of the Humidifier Dehumidifier Accessory.
#[derive(Default)]
pub struct HumidifierDehumidifierInner {
    /// ID of the Humidifier Dehumidifier Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Humidifier Dehumidifier Service.
    pub humidifier_dehumidifier: humidifier_dehumidifier::HumidifierDehumidifier,
}

impl HapAccessory for HumidifierDehumidifierInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&HapAccessoryService> {
        vec![
            &self.accessory_information,
            &self.humidifier_dehumidifier,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.humidifier_dehumidifier,
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

/// Creates a new Humidifier Dehumidifier Accessory.
pub fn new(information: Information) -> Result<HumidifierDehumidifier> {
    let mut humidifier_dehumidifier = humidifier_dehumidifier::new();
    humidifier_dehumidifier.set_primary(true);
    Ok(HumidifierDehumidifier::new(HumidifierDehumidifierInner {
        accessory_information: information.to_service()?,
        humidifier_dehumidifier,
        ..Default::default()
    }))
}
