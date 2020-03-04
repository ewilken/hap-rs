use crate::{
    accessory::{Accessory, HapAccessory, HapAccessoryService, Information},
    pointer,
    service::{accessory_information::AccessoryInformation, camera_rtp_stream_management, microphone, HapService},
    Result,
};

/// IP Camera Accessory.
pub type IpCamera = Accessory<IpCameraInner>;

/// Inner type of the IP Camera Accessory.
#[derive(Default)]
pub struct IpCameraInner {
    /// ID of the IP Camera Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Camera RTP Stream Management Service.
    pub camera_rtp_stream_management: camera_rtp_stream_management::CameraRTPStreamManagement,
    /// Microphone Service.
    pub microphone: microphone::Microphone,
}

impl HapAccessory for IpCameraInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> {
        vec![
            &self.accessory_information,
            &self.camera_rtp_stream_management,
            &self.microphone,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.camera_rtp_stream_management,
            &mut self.microphone,
        ]
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

/// Creates a new IP Camera Accessory.
pub fn new(information: Information) -> Result<IpCamera> {
    let mut camera_rtp_stream_management = camera_rtp_stream_management::new();
    camera_rtp_stream_management.set_primary(true);
    Ok(IpCamera::new(IpCameraInner {
        accessory_information: information.to_service()?,
        camera_rtp_stream_management,
        microphone: microphone::new(),
        ..Default::default()
    }))
}
