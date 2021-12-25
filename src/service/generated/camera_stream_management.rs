// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		selected_stream_configuration::SelectedStreamConfigurationCharacteristic,
		setup_endpoint::SetupEndpointCharacteristic,
		streaming_status::StreamingStatusCharacteristic,
		supported_audio_stream_configuration::SupportedAudioStreamConfigurationCharacteristic,
		supported_rtp_configuration::SupportedRtpConfigurationCharacteristic,
		supported_video_stream_configuration::SupportedVideoStreamConfigurationCharacteristic,
		active::ActiveCharacteristic,
	},
    HapType,
};

/// Camera Stream Management service.
#[derive(Debug, Default)]
pub struct CameraStreamManagementService {
    /// Instance ID of the Camera Stream Management service.
    id: u64,
    /// [`HapType`](HapType) of the Camera Stream Management service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

	/// Selected Stream Configuration characteristic (required).
	pub selected_stream_configuration: SelectedStreamConfigurationCharacteristic,
	/// Setup Endpoint characteristic (required).
	pub setup_endpoint: SetupEndpointCharacteristic,
	/// Streaming Status characteristic (required).
	pub streaming_status: StreamingStatusCharacteristic,
	/// Supported Audio Stream Configuration characteristic (required).
	pub supported_audio_stream_configuration: SupportedAudioStreamConfigurationCharacteristic,
	/// Supported RTP Configuration characteristic (required).
	pub supported_rtp_configuration: SupportedRtpConfigurationCharacteristic,
	/// Supported Video Stream Configuration characteristic (required).
	pub supported_video_stream_configuration: SupportedVideoStreamConfigurationCharacteristic,

	/// Active characteristic (optional).
	pub active: Option<ActiveCharacteristic>,
}

impl CameraStreamManagementService {
    /// Creates a new Camera Stream Management service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::CameraStreamManagement,
			selected_stream_configuration: SelectedStreamConfigurationCharacteristic::new(id + 1 + 0, accessory_id),
			setup_endpoint: SetupEndpointCharacteristic::new(id + 1 + 1, accessory_id),
			streaming_status: StreamingStatusCharacteristic::new(id + 1 + 2, accessory_id),
			supported_audio_stream_configuration: SupportedAudioStreamConfigurationCharacteristic::new(id + 1 + 3, accessory_id),
			supported_rtp_configuration: SupportedRtpConfigurationCharacteristic::new(id + 1 + 4, accessory_id),
			supported_video_stream_configuration: SupportedVideoStreamConfigurationCharacteristic::new(id + 1 + 5, accessory_id),
			active: Some(ActiveCharacteristic::new(id + 1 + 0 + 6, accessory_id)),
			..Default::default()
        }
    }
}

impl HapService for CameraStreamManagementService {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn set_type(&mut self, hap_type: HapType) {
        self.hap_type = hap_type;
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    fn get_linked_services(&self) -> Vec<u64> {
        self.linked_services.clone()
    }

    fn set_linked_services(&mut self, linked_services: Vec<u64>) {
        self.linked_services = linked_services;
    }

    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
        for characteristic in self.get_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
        for characteristic in self.get_mut_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
			&self.selected_stream_configuration,
			&self.setup_endpoint,
			&self.streaming_status,
			&self.supported_audio_stream_configuration,
			&self.supported_rtp_configuration,
			&self.supported_video_stream_configuration,
		];
		if let Some(c) = &self.active {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.selected_stream_configuration,
			&mut self.setup_endpoint,
			&mut self.streaming_status,
			&mut self.supported_audio_stream_configuration,
			&mut self.supported_rtp_configuration,
			&mut self.supported_video_stream_configuration,
		];
		if let Some(c) = &mut self.active {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for CameraStreamManagementService {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}
