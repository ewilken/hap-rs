use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    accessory::{AccessoryInformation, HapAccessory},
    service::{
        accessory_information::AccessoryInformationService,
        speaker::SpeakerService,
        television::TelevisionService,
        HapService,
    },
    HapType,
    Result,
};

/// Television Accessory.
#[derive(Default)]
pub struct TelevisionAccessory {
    /// ID of the Television Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformationService,
    /// Television Service.
    pub television: TelevisionService,
    /// Speaker Service.
    pub speaker: SpeakerService,
}

impl TelevisionAccessory {
    /// Creates a new Television Accessory.
    pub fn new(id: u64, information: AccessoryInformation) -> Result<Self> {
        let accessory_information = information.to_service(1, id)?;

        let television_id = 2 + accessory_information.get_characteristics().len() as u64;
        let mut television = TelevisionService::new(television_id, id);
        television.set_primary(true);

        // TODO - figure out how to auto-set reasonable default values for tlv8 characteristics
        television.display_order = None;

        let speaker_id = 3 + television_id + television.get_characteristics().len() as u64;
        let speaker = SpeakerService::new(speaker_id, id);

        Ok(Self {
            id,
            accessory_information,
            television,
            speaker,
        })
    }
}

impl HapAccessory for TelevisionAccessory {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_service(&self, hap_type: HapType) -> Option<&dyn HapService> {
        for service in self.get_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_mut_service(&mut self, hap_type: HapType) -> Option<&mut dyn HapService> {
        for service in self.get_mut_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_services(&self) -> Vec<&dyn HapService> {
        vec![&self.accessory_information, &self.television, &self.speaker]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
        vec![&mut self.accessory_information, &mut self.television, &mut self.speaker]
    }
}

impl Serialize for TelevisionAccessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}
