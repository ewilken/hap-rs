// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		supported_video_stream_configuration,
		supported_audio_stream_configuration,
		supported_rtp_configuration,
		selected_rtp_stream_configuration,
		streaming_status,
		setup_endpoints,
		name,
	},
    HapType,
};

/// Camera RTP Stream Management Service.
pub type CameraRTPStreamManagement = Service<CameraRTPStreamManagementInner>;

impl Default for CameraRTPStreamManagement {
    fn default() -> CameraRTPStreamManagement { new() }
}

/// Inner type of the Camera RTP Stream Management Service.
#[derive(Default)]
pub struct CameraRTPStreamManagementInner {
    /// ID of the Camera RTP Stream Management Service.
    id: u64,
    /// `HapType` of the Camera RTP Stream Management Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Supported Video Stream Configuration Characteristic.
	pub supported_video_stream_configuration: supported_video_stream_configuration::SupportedVideoStreamConfiguration,
	/// Supported Audio Stream Configuration Characteristic.
	pub supported_audio_stream_configuration: supported_audio_stream_configuration::SupportedAudioStreamConfiguration,
	/// Supported RTP Configuration Characteristic.
	pub supported_rtp_configuration: supported_rtp_configuration::SupportedRTPConfiguration,
	/// Selected RTP Stream Configuration Characteristic.
	pub selected_rtp_stream_configuration: selected_rtp_stream_configuration::SelectedRTPStreamConfiguration,
	/// Streaming Status Characteristic.
	pub streaming_status: streaming_status::StreamingStatus,
	/// Setup Endpoints Characteristic.
	pub setup_endpoints: setup_endpoints::SetupEndpoints,

	/// Name Characteristic.
	pub name: Option<name::Name>,
}

impl HapService for CameraRTPStreamManagementInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_type(&self) -> HapType {
        self.hap_type
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

    fn get_characteristics(&self) -> Vec<&HapCharacteristic> {
        let mut characteristics: Vec<&HapCharacteristic> = vec![
			&self.supported_video_stream_configuration,
			&self.supported_audio_stream_configuration,
			&self.supported_rtp_configuration,
			&self.selected_rtp_stream_configuration,
			&self.streaming_status,
			&self.setup_endpoints,
		];
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> {
        let mut characteristics: Vec<&mut HapCharacteristic> = vec![
			&mut self.supported_video_stream_configuration,
			&mut self.supported_audio_stream_configuration,
			&mut self.supported_rtp_configuration,
			&mut self.selected_rtp_stream_configuration,
			&mut self.streaming_status,
			&mut self.setup_endpoints,
		];
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Camera RTP Stream Management Service.
pub fn new() -> CameraRTPStreamManagement {
    CameraRTPStreamManagement::new(CameraRTPStreamManagementInner {
        hap_type: HapType::CameraRTPStreamManagement,
		supported_video_stream_configuration: supported_video_stream_configuration::new(),
		supported_audio_stream_configuration: supported_audio_stream_configuration::new(),
		supported_rtp_configuration: supported_rtp_configuration::new(),
		selected_rtp_stream_configuration: selected_rtp_stream_configuration::new(),
		streaming_status: streaming_status::new(),
		setup_endpoints: setup_endpoints::new(),
		..Default::default()
    })
}
