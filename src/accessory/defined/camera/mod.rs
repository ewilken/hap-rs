use crate::{accessory::{AccessoryInformation, HapAccessory}, characteristic::HapCharacteristic, service::{accessory_information::AccessoryInformationService, camera_stream_management::CameraStreamManagementService, microphone::MicrophoneService, HapService}, HapType, Result};

use futures::executor;
use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

pub mod protocol;
pub mod manager;
pub mod media;

/// Camera Accessory.
#[derive(Default, Debug)]
pub struct CameraAccessory {
    /// ID of the Foo accessory.
    id: u64,

    /// Accessory Information service.
    pub accessory_information: AccessoryInformationService,

    /// Microphone
    pub microphone: MicrophoneService,

    /// Camera streams
    pub streams: Vec<CameraStreamManagementService>,
}

impl HapAccessory for CameraAccessory {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

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
        let mut services: Vec<&dyn HapService> =
            vec![&self.accessory_information, &self.microphone];
        services.append(&mut self.streams.iter().map(|s| s as &dyn HapService).collect());
        services
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
        let mut services: Vec<&mut dyn HapService> =
            vec![&mut self.accessory_information, &mut self.microphone];
        services.append(
            &mut self
                .streams
                .iter_mut()
                .map(|s| s as &mut dyn HapService)
                .collect(),
        );
        services
    }
}

impl Serialize for CameraAccessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}

impl CameraAccessory {
    /// Creates a new Camera Accessory.
    pub fn new(id: u64, streams_count: usize, information: AccessoryInformation) -> Result<Self> {
        let accessory_information = information.to_service(1, id)?;

        let microphone_id = 2 + accessory_information.get_characteristics().len() as u64;
        let mut microphone = MicrophoneService::new(microphone_id, id);

        executor::block_on(microphone.mute.set_value(0.into()))?;

        // create streams
        let mut stream_id = 3 + microphone_id + microphone.get_characteristics().len() as u64;
        let mut streams = vec![];
        for _ in 0..streams_count {
            let s = CameraStreamManagementService::new(stream_id, id);
            stream_id += 1 + s.get_characteristics().len() as u64;
            streams.push(s);
        }

        Ok(Self {
            id,
            accessory_information,
            microphone,
            streams,
        })
    }
}