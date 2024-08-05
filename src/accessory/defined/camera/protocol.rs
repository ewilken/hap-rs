use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::tlv::{decode, encode};
use rand::RngCore;
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

pub const SETUP_TYPES_SESSION_ID: u8 = 0x01;
pub const SETUP_TYPES_STATUS: u8 = 0x02;
pub const SETUP_TYPES_ADDRESS: u8 = 0x03;
pub const SETUP_TYPES_VIDEO_SRTP_PARAM: u8 = 0x04;
pub const SETUP_TYPES_AUDIO_SRTP_PARAM: u8 = 0x05;
pub const SETUP_TYPES_VIDEO_SSRC: u8 = 0x06;
pub const SETUP_TYPES_AUDIO_SSRC: u8 = 0x07;

pub const SETUP_STATUS_SUCCESS: u8 = 0x00;
pub const SETUP_STATUS_BUSY: u8 = 0x01;
pub const SETUP_STATUS_ERROR: u8 = 0x02;

pub const SETUP_IPV_IPV4: u8 = 0x00;
pub const SETUP_IPV_IPV6: u8 = 0x01;

pub const SETUP_ADDR_INFO_ADDRESS_VER: u8 = 0x01;
pub const SETUP_ADDR_INFO_ADDRESS: u8 = 0x02;
pub const SETUP_ADDR_INFO_VIDEO_RTP_PORT: u8 = 0x03;
pub const SETUP_ADDR_INFO_AUDIO_RTP_PORT: u8 = 0x04;

pub const SETUP_SRTP_PARAM_CRYPTO: u8 = 0x01;
pub const SETUP_SRTP_PARAM_MASTER_KEY: u8 = 0x02;
pub const SETUP_SRTP_PARAM_MASTER_SALT: u8 = 0x03;

pub const STREAMING_STATUS_AVAILABLE: u8 = 0x00;
pub const STREAMING_STATUS_STREAMING: u8 = 0x01;
pub const STREAMING_STATUS_BUSY: u8 = 0x02;

pub const RTP_CONFIG_TYPES_CRYPTO: u8 = 0x02;

pub const SRTP_CRYPTO_SUITES_AES_CM_128_HMAC_SHA1_80: u8 = 0x00;
pub const SRTP_CRYPTO_SUITES_AES_CM_256_HMAC_SHA1_80: u8 = 0x01;
pub const SRTP_CRYPTO_SUITES_NONE: u8 = 0x02;

pub const VIDEO_TYPES_CODEC: u8 = 0x01;
pub const VIDEO_TYPES_CODEC_PARAM: u8 = 0x02;
pub const VIDEO_TYPES_ATTRIBUTES: u8 = 0x03;
pub const VIDEO_TYPES_RTP_PARAM: u8 = 0x04;

pub const VIDEO_CODEC_TYPES_H264: u8 = 0x00;

pub const VIDEO_CODEC_PARAM_TYPES_PROFILE_ID: u8 = 0x01;
pub const VIDEO_CODEC_PARAM_TYPES_LEVEL: u8 = 0x02;
pub const VIDEO_CODEC_PARAM_TYPES_PACKETIZATION_MODE: u8 = 0x03;
pub const VIDEO_CODEC_PARAM_TYPES_CVO_ENABLED: u8 = 0x04;
pub const VIDEO_CODEC_PARAM_TYPES_CVO_ID: u8 = 0x05;

pub const VIDEO_CODEC_PARAM_CVO_TYPES_UNSUPPORTED: u8 = 0x01;
pub const VIDEO_CODEC_PARAM_CVO_TYPES_SUPPORTED: u8 = 0x02;

pub const VIDEO_CODEC_PARAM_PROFILE_ID_TYPES_BASELINE: u8 = 0x00;
pub const VIDEO_CODEC_PARAM_PROFILE_ID_TYPES_MAIN: u8 = 0x01;
pub const VIDEO_CODEC_PARAM_PROFILE_ID_TYPES_HIGH: u8 = 0x02;

pub const VIDEO_CODEC_PARAM_LEVEL_TYPES_TYPE3_1: u8 = 0x00;
pub const VIDEO_CODEC_PARAM_LEVEL_TYPES_TYPE3_2: u8 = 0x01;
pub const VIDEO_CODEC_PARAM_LEVEL_TYPES_TYPE4_0: u8 = 0x02;

pub const VIDEO_CODEC_PARAM_PACKETIZATION_MODE_TYPES_NON_INTERLEAVED: u8 = 0x00;

pub const VIDEO_ATTRIBUTES_TYPES_IMAGE_WIDTH: u8 = 0x01;
pub const VIDEO_ATTRIBUTES_TYPES_IMAGE_HEIGHT: u8 = 0x02;
pub const VIDEO_ATTRIBUTES_TYPES_FRAME_RATE: u8 = 0x03;

pub const SUPPORTED_VIDEO_CONFIG_TAG: u8 = 0x01;

pub const SELECTED_STREAM_CONFIGURATION_TYPES_SESSION: u8 = 0x01;
pub const SELECTED_STREAM_CONFIGURATION_TYPES_VIDEO: u8 = 0x02;
pub const SELECTED_STREAM_CONFIGURATION_TYPES_AUDIO: u8 = 0x03;

pub const RTP_PARAM_TYPES_PAYLOAD_TYPE: u8 = 0x01;
pub const RTP_PARAM_TYPES_SYNCHRONIZATION_SOURCE: u8 = 0x02;
pub const RTP_PARAM_TYPES_MAX_BIT_RATE: u8 = 0x03;
pub const RTP_PARAM_TYPES_RTCP_SEND_INTERVAL: u8 = 0x04;
pub const RTP_PARAM_TYPES_MAX_MTU: u8 = 0x05;
pub const RTP_PARAM_TYPES_COMFORT_NOISE_PAYLOAD_TYPE: u8 = 0x06;

pub const AUDIO_TYPES_CODEC: u8 = 0x01;
pub const AUDIO_TYPES_CODEC_PARAM: u8 = 0x02;
pub const AUDIO_TYPES_RTP_PARAM: u8 = 0x03;
pub const AUDIO_TYPES_COMFORT_NOISE: u8 = 0x04;

pub const AUDIO_CODEC_TYPES_PCMU: u8 = 0x00;
pub const AUDIO_CODEC_TYPES_PCMA: u8 = 0x01;
pub const AUDIO_CODEC_TYPES_AACELD: u8 = 0x02;
pub const AUDIO_CODEC_TYPES_OPUS: u8 = 0x03;

pub const AUDIO_CODEC_PARAM_TYPES_CHANNEL: u8 = 0x01;
pub const AUDIO_CODEC_PARAM_TYPES_BIT_RATE: u8 = 0x02;
pub const AUDIO_CODEC_PARAM_TYPES_SAMPLE_RATE: u8 = 0x03;
pub const AUDIO_CODEC_PARAM_TYPES_PACKET_TIME: u8 = 0x04;

pub const AUDIO_CODEC_PARAM_BIT_RATE_TYPES_VARIABLE: u8 = 0x00;
pub const AUDIO_CODEC_PARAM_BIT_RATE_TYPES_CONSTANT: u8 = 0x01;

pub const AUDIO_CODEC_PARAM_SAMPLE_RATE_TYPES_KHZ_8: u8 = 0x00;
pub const AUDIO_CODEC_PARAM_SAMPLE_RATE_TYPES_KHZ_16: u8 = 0x01;
pub const AUDIO_CODEC_PARAM_SAMPLE_RATE_TYPES_KHZ_24: u8 = 0x02;

pub const SUPPORTED_AUDIO_CODECS_TAG: u8 = 0x01;
pub const SUPPORTED_COMFORT_NOISE_TAG: u8 = 0x02;
pub const SUPPORTED_AUDIO_CONFIG_TAG: u8 = 0x02;
pub const SET_CONFIG_REQUEST_TAG: u8 = 0x02;
pub const SESSION_ID: u8 = 0x01;

pub const NO_SRTP: [u8; 7] = [0x01, 0x01, 0x02, 0x02, 0x00, 0x03, 0x00];

#[derive(Debug)]
pub struct SetupEndpointRequest {
    pub session_id: Uuid,
    pub ip: IpAddr,
    pub video_port: u16,
    pub audio_port: u16,
    pub video_crypto: SetupEndpointCrypto,
    pub audio_crypto: SetupEndpointCrypto,
}

#[derive(Debug)]
pub struct SelectedStreamRequest {
    pub session_id: Uuid,
    pub kind: SelectedStreamRequestKind,
}

#[derive(Debug)]
pub enum SelectedStreamRequestKind {
    Start,
    Stop,
    StartAndReconfigure,
}

#[derive(Debug, Clone)]
pub struct SetupEndpointCrypto {
    pub cypto_suite: u8,
    pub master_key: Vec<u8>,
    pub master_salt: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Property cannot be found")]
    MissingProperty,
    #[error("Cannot convert to array")]
    ArrayConversion,
    #[error("Cannot parse string to utf8")]
    Utf8,
    #[error("Ip cannot be parsed: {0}")]
    Ip(String),
}

#[derive(Debug)]
pub struct StreamInfo {
    pub session_id: Uuid,
    pub video_codec: Option<StreamVideoCodec>,
    pub video_attributes: Option<StreamVideoAttributes>,
    pub video_rtp: Option<StreamVideoRtp>,
    pub audio: Option<StreamAudio>,
}

impl StreamInfo {
    pub fn new(session_id: Uuid) -> Self {
        Self {
            session_id,
            video_codec: Default::default(),
            video_attributes: Default::default(),
            video_rtp: Default::default(),
            audio: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct StreamVideoAttributes {
    pub width: u16,
    pub height: u16,
    pub fps: u8,
}

#[derive(Debug)]
pub struct StreamVideoCodec {
    pub profile_id: Vec<u8>,
    pub level: Vec<u8>,
}

#[derive(Debug)]
pub struct StreamVideoRtp {
    pub ssrc: Option<u32>,
    pub payload_type: Option<Vec<u8>>,
    pub max_bitrate: Option<u16>,
    pub rtcp_interval: Option<f32>,
    pub max_mtu: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct StreamAudio {
    pub codec: Vec<u8>,
    pub comfort_noise: Vec<u8>,

    pub channel: u8,
    pub bitrate: bool,
    pub sample_rate: u8,
    pub packet_time: u8,

    pub ssrc: u32,
    pub payload_type: Vec<u8>,
    pub max_bitrate: u16,
    pub rtcp_interval: f32,
    pub comfort_payload_type: Vec<u8>,
}

#[derive(Debug)]
pub struct EndpointResponse {
    pub tlv: Vec<u8>,
    pub video_ssrc: u32,
    pub audio_ssrc: u32,
}

pub(crate) fn setup_endpoint_response(request: &SetupEndpointRequest, address: &IpAddr) -> EndpointResponse {
    let (video_srtp_tlv, audio_srtp_tlv) = if true {
        let video = encode(vec![
            (
                SETUP_SRTP_PARAM_CRYPTO,
                vec![SRTP_CRYPTO_SUITES_AES_CM_128_HMAC_SHA1_80],
            ),
            (SETUP_SRTP_PARAM_MASTER_KEY, request.video_crypto.master_key.clone()),
            (SETUP_SRTP_PARAM_MASTER_SALT, request.video_crypto.master_salt.clone()),
        ]);

        let audio = encode(vec![
            (
                SETUP_SRTP_PARAM_CRYPTO,
                vec![SRTP_CRYPTO_SUITES_AES_CM_128_HMAC_SHA1_80],
            ),
            (SETUP_SRTP_PARAM_MASTER_KEY, request.audio_crypto.master_key.clone()),
            (SETUP_SRTP_PARAM_MASTER_SALT, request.audio_crypto.master_salt.clone()),
        ]);

        (video, audio)
    } else {
        (NO_SRTP.to_vec(), NO_SRTP.to_vec())
    };

    let mut data = [0u8; 4];
    rand::thread_rng().fill_bytes(&mut data);
    let video_ssrc = u32::from_le_bytes(data);
    rand::thread_rng().fill_bytes(&mut data);
    let audio_ssrc = u32::from_le_bytes(data);

    let res_address_tlv = encode(vec![
        (
            SETUP_ADDR_INFO_ADDRESS_VER,
            vec![if address.is_ipv6() { 0x01 } else { 0x00 }],
        ),
        (SETUP_ADDR_INFO_ADDRESS, address.to_string().as_bytes().to_vec()),
        (
            SETUP_ADDR_INFO_VIDEO_RTP_PORT,
            request.video_port.to_le_bytes().to_vec(),
        ),
        (
            SETUP_ADDR_INFO_AUDIO_RTP_PORT,
            request.audio_port.to_le_bytes().to_vec(),
        ),
    ]);

    let tlv = encode(vec![
        (
            SETUP_TYPES_SESSION_ID,
            request.session_id.to_u128_le().to_le_bytes().to_vec(),
        ),
        (SETUP_TYPES_STATUS, vec![SETUP_STATUS_SUCCESS]),
        (SETUP_TYPES_ADDRESS, res_address_tlv),
        (SETUP_TYPES_VIDEO_SRTP_PARAM, video_srtp_tlv),
        (SETUP_TYPES_AUDIO_SRTP_PARAM, audio_srtp_tlv),
        (SETUP_TYPES_VIDEO_SSRC, video_ssrc.to_le_bytes().to_vec()),
        (SETUP_TYPES_AUDIO_SSRC, audio_ssrc.to_le_bytes().to_vec()),
    ]);

    EndpointResponse {
        tlv,
        video_ssrc,
        audio_ssrc,
    }
}

pub(crate) fn setup_endpoint_request(bytes: &[u8]) -> std::result::Result<SetupEndpointRequest, ProtocolError> {
    let objs = decode(bytes);

    let session_id = objs
        .get(&SETUP_TYPES_SESSION_ID)
        .ok_or(ProtocolError::MissingProperty)?;

    let session_id = Uuid::from_bytes(
        session_id
            .clone()
            .try_into()
            .map_err(|_| ProtocolError::ArrayConversion)?,
    );

    let address_tlv = objs.get(&SETUP_TYPES_ADDRESS).ok_or(ProtocolError::MissingProperty)?;
    let address = decode(address_tlv);

    let is_ipv6 = address
        .get(&SETUP_ADDR_INFO_ADDRESS_VER)
        .ok_or(ProtocolError::MissingProperty)?
        .first()
        .ok_or(ProtocolError::MissingProperty)?;

    let is_ipv6 = *is_ipv6 != 0;

    let addr = address
        .get(&SETUP_ADDR_INFO_ADDRESS)
        .ok_or(ProtocolError::MissingProperty)?;
    let plain = std::str::from_utf8(addr).map_err(|_| ProtocolError::Utf8)?;

    let ip = if is_ipv6 {
        IpAddr::V6(Ipv6Addr::from_str(plain).map_err(|_| ProtocolError::Ip(plain.to_string()))?)
    } else {
        IpAddr::V4(Ipv4Addr::from_str(plain).map_err(|_| ProtocolError::Ip(plain.to_string()))?)
    };

    let target_video_port = address
        .get(&SETUP_ADDR_INFO_VIDEO_RTP_PORT)
        .ok_or(ProtocolError::MissingProperty)?;
    let video_port = u16::from_le_bytes(
        target_video_port
            .clone()
            .try_into()
            .map_err(|_| ProtocolError::ArrayConversion)?,
    );

    let target_audio_port = address
        .get(&SETUP_ADDR_INFO_AUDIO_RTP_PORT)
        .ok_or(ProtocolError::MissingProperty)?;
    let audio_port = u16::from_le_bytes(
        target_audio_port
            .clone()
            .try_into()
            .map_err(|_| ProtocolError::ArrayConversion)?,
    );

    // video srtp params
    let video_srtp_tlv = objs
        .get(&SETUP_TYPES_VIDEO_SRTP_PARAM)
        .ok_or(ProtocolError::MissingProperty)?;

    let video_srtp = decode(video_srtp_tlv);
    let video_crypto_suite = video_srtp
        .get(&SETUP_SRTP_PARAM_CRYPTO)
        .ok_or(ProtocolError::MissingProperty)?
        .first()
        .ok_or(ProtocolError::MissingProperty)?;

    let video_master_key = video_srtp
        .get(&SETUP_SRTP_PARAM_MASTER_KEY)
        .ok_or(ProtocolError::MissingProperty)?;
    let video_master_salt = video_srtp
        .get(&SETUP_SRTP_PARAM_MASTER_SALT)
        .ok_or(ProtocolError::MissingProperty)?;

    let video_crypto = SetupEndpointCrypto {
        cypto_suite: *video_crypto_suite,
        master_key: video_master_key.clone(),
        master_salt: video_master_salt.clone(),
    };

    // audio srtp params
    let audio_srtp_tlv = objs
        .get(&SETUP_TYPES_AUDIO_SRTP_PARAM)
        .ok_or(ProtocolError::MissingProperty)?;
    let audio_srtp = decode(audio_srtp_tlv);
    let audio_crypto_suite = audio_srtp.get(&SETUP_SRTP_PARAM_CRYPTO).unwrap().first().unwrap();
    let audio_master_key = audio_srtp
        .get(&SETUP_SRTP_PARAM_MASTER_KEY)
        .ok_or(ProtocolError::MissingProperty)?;
    let audio_master_salt = audio_srtp
        .get(&SETUP_SRTP_PARAM_MASTER_SALT)
        .ok_or(ProtocolError::MissingProperty)?;

    let audio_crypto = SetupEndpointCrypto {
        cypto_suite: *audio_crypto_suite,
        master_key: audio_master_key.clone(),
        master_salt: audio_master_salt.clone(),
    };

    Ok(SetupEndpointRequest {
        session_id,
        ip,
        video_port,
        audio_port,
        video_crypto,
        audio_crypto,
    })
}

pub(crate) fn supported_rtp_config(enabled: bool) -> Vec<u8> {
    let support_srtp = if enabled {
        SRTP_CRYPTO_SUITES_AES_CM_128_HMAC_SHA1_80
    } else {
        SRTP_CRYPTO_SUITES_NONE
    };

    encode(vec![(RTP_CONFIG_TYPES_CRYPTO, vec![support_srtp])])
}

pub(crate) fn video_stream_config(data: serde_json::Value) -> Vec<u8> {
    // profiles
    let mut profiles = if let serde_json::Value::Array(arr) = &data["codec"]["profiles"] {
        let mut result = arr
            .iter()
            .filter_map(|v| match v {
                serde_json::Value::Number(n) => n.as_u64().map(|u| u as u8),
                _ => None,
            })
            .map(|profile| {
                let mut bytes = encode(vec![(0x01, vec![profile])]);
                bytes.push(0x00);
                bytes.push(0x00);
                bytes
            })
            .flatten()
            .collect::<Vec<u8>>();

        result.pop();
        result.pop();

        result
    } else {
        vec![]
    };

    // levels
    let mut levels = if let serde_json::Value::Array(arr) = &data["codec"]["levels"] {
        let mut result = arr
            .iter()
            .filter_map(|v| match v {
                serde_json::Value::Number(n) => n.as_u64().map(|u| u as u8),
                _ => None,
            })
            .map(|lvl| {
                let mut bytes = encode(vec![(0x02, vec![lvl])]);
                bytes.push(0x00);
                bytes.push(0x00);
                bytes
            })
            .flatten()
            .collect::<Vec<u8>>();

        result.pop();
        result.pop();

        result
    } else {
        vec![]
    };

    let mut codec_params_tlv = vec![];
    codec_params_tlv.append(&mut profiles);
    codec_params_tlv.append(&mut levels);
    codec_params_tlv.append(&mut encode(vec![(0x03, vec![0x00])]));

    // resolutions
    let mut attr_tlv = vec![];
    if let serde_json::Value::Array(arr) = &data["resolutions"] {
        let u16_list = arr
            .iter()
            .filter_map(|v| match v {
                serde_json::Value::Array(ref list) => {
                    let dims = list
                        .iter()
                        .filter_map(|v| match v {
                            serde_json::Value::Number(n) => n.as_u64().map(|u| u as u16),
                            _ => None,
                        })
                        .collect::<Vec<u16>>();

                    if let [a, b, c] = dims[..] {
                        Some((a, b, c))
                    } else {
                        None
                    }
                },
                _ => None,
            })
            .collect::<Vec<_>>();

        for (width, height, rate) in u16_list {
            let res_tlv = encode(vec![
                (0x01, width.to_le_bytes().to_vec()),
                (0x02, height.to_le_bytes().to_vec()),
                (0x03, (rate as u8).to_le_bytes().to_vec()),
            ]);

            attr_tlv.append(&mut encode(vec![(0x03, res_tlv)]));

            // delimiter
            attr_tlv.push(0x00);
            attr_tlv.push(0x00);
        }

        attr_tlv.pop();
        attr_tlv.pop();
    }

    let mut config_tlv = encode(vec![(0x01, vec![0x00]), (0x02, codec_params_tlv)]);

    config_tlv.append(&mut attr_tlv);

    encode(vec![(0x01, config_tlv)])
}

pub(crate) fn audio_stream_config(data: serde_json::Value) -> Vec<u8> {
    #[derive(serde::Deserialize)]
    struct Codec {
        r#type: String,
        samplerate: u8,
    }

    let mut configs = vec![];

    // codecs
    let codecs = serde_json::from_value::<Vec<Codec>>(data["codecs"].clone()).unwrap();
    for codec in codecs {
        let (c, bitrate) = match codec.r#type.as_str() {
            "OPUS" => (0x03, 0x00u8),
            "AAC-eld" => (0x02, 0x00),
            _ => panic!("unsupported codec"),
        };

        let samplerate = match codec.samplerate {
            8u8 => 0x00u8,
            16 => 0x01,
            24 => 0x02,
            _ => panic!("unsupported samplerate"),
        };

        let param_tlv = encode(vec![
            (0x01, vec![0x01]),
            (0x02, bitrate.to_le_bytes().to_vec()),
            (0x03, samplerate.to_le_bytes().to_vec()),
        ]);

        let config_tlv = encode(vec![(0x01, vec![c]), (0x02, param_tlv)]);

        configs.append(&mut encode(vec![(0x01, config_tlv)]));
    }

    // todo: handle panic

    configs.append(&mut encode(vec![(0x02, vec![0x00])])); // comfort noise; 0x00 - false, 0x01 - true

    configs
}

pub(crate) fn stream_info_response(bytes: &[u8]) -> std::result::Result<StreamInfo, ProtocolError> {
    let objs = decode(bytes);
    let session_objs = decode(
        objs.get(&SELECTED_STREAM_CONFIGURATION_TYPES_SESSION)
            .ok_or(ProtocolError::MissingProperty)?,
    );

    let session_id = session_objs
        .get(&SETUP_TYPES_SESSION_ID)
        .ok_or(ProtocolError::MissingProperty)?;

    let session_id = Uuid::from_bytes(
        session_id
            .clone()
            .try_into()
            .map_err(|_| ProtocolError::ArrayConversion)?,
    );

    let mut info = StreamInfo::new(session_id);

    // video
    if let Some(video_tlv) = objs.get(&SELECTED_STREAM_CONFIGURATION_TYPES_VIDEO) {
        let video_objs = decode(&video_tlv);

        if let Some(video_codec_params_tlv) = video_objs.get(&VIDEO_TYPES_CODEC_PARAM) {
            let video_codec_params_objs = decode(&video_codec_params_tlv);

            info.video_codec = Some(StreamVideoCodec {
                profile_id: video_codec_params_objs
                    .get(&VIDEO_CODEC_PARAM_TYPES_PROFILE_ID)
                    .ok_or(ProtocolError::MissingProperty)?
                    .clone(),
                level: video_codec_params_objs
                    .get(&VIDEO_CODEC_PARAM_TYPES_LEVEL)
                    .ok_or(ProtocolError::MissingProperty)?
                    .clone(),
            });
        }

        if let Some(video_attrs_tlv) = video_objs.get(&VIDEO_TYPES_ATTRIBUTES) {
            let video_attrs_objs = decode(&video_attrs_tlv);

            info.video_attributes = Some(StreamVideoAttributes {
                width: u16::from_le_bytes(
                    video_attrs_objs
                        .get(&VIDEO_ATTRIBUTES_TYPES_IMAGE_WIDTH)
                        .ok_or(ProtocolError::MissingProperty)?
                        .clone()
                        .try_into()
                        .map_err(|_| ProtocolError::ArrayConversion)?,
                ),
                height: u16::from_le_bytes(
                    video_attrs_objs
                        .get(&VIDEO_ATTRIBUTES_TYPES_IMAGE_HEIGHT)
                        .ok_or(ProtocolError::MissingProperty)?
                        .clone()
                        .try_into()
                        .map_err(|_| ProtocolError::ArrayConversion)?,
                ),
                fps: video_attrs_objs
                    .get(&VIDEO_ATTRIBUTES_TYPES_FRAME_RATE)
                    .ok_or(ProtocolError::MissingProperty)?
                    .first()
                    .cloned()
                    .ok_or(ProtocolError::MissingProperty)?,
            });
        }

        if let Some(video_rtp_param_tlv) = video_objs.get(&VIDEO_TYPES_RTP_PARAM) {
            let video_rtp_param_objs = decode(&video_rtp_param_tlv);

            info.video_rtp = Some(StreamVideoRtp {
                ssrc: video_rtp_param_objs
                    .get(&RTP_PARAM_TYPES_SYNCHRONIZATION_SOURCE)
                    .map(|bytes| bytes.clone().try_into() as Result<[u8; 4], _>)
                    .map_or(Ok(None), |v| v.map(Some))
                    .map_err(|_| ProtocolError::ArrayConversion)?
                    .map(u32::from_le_bytes),
                payload_type: video_rtp_param_objs.get(&RTP_PARAM_TYPES_PAYLOAD_TYPE).cloned(),
                max_bitrate: video_rtp_param_objs
                    .get(&RTP_PARAM_TYPES_MAX_BIT_RATE)
                    .map(|bytes| bytes.clone().try_into() as Result<[u8; 2], _>)
                    .map_or(Ok(None), |v| v.map(Some))
                    .map_err(|_| ProtocolError::ArrayConversion)?
                    .map(u16::from_le_bytes),
                rtcp_interval: video_rtp_param_objs
                    .get(&RTP_PARAM_TYPES_RTCP_SEND_INTERVAL)
                    .map(|bytes| bytes.clone().try_into() as Result<[u8; 4], _>)
                    .map_or(Ok(None), |v| v.map(Some))
                    .map_err(|_| ProtocolError::ArrayConversion)?
                    .map(f32::from_le_bytes),
                max_mtu: video_rtp_param_objs.get(&RTP_PARAM_TYPES_RTCP_SEND_INTERVAL).cloned(),
            });
        }
    }

    // audio
    if let Some(audio_tlv) = objs.get(&SELECTED_STREAM_CONFIGURATION_TYPES_AUDIO) {
        let audio_objs = decode(&audio_tlv);
        let codec = audio_objs
            .get(&AUDIO_TYPES_CODEC)
            .ok_or(ProtocolError::MissingProperty)?
            .clone();

        let audio_codec_param_objs = decode(
            audio_objs
                .get(&AUDIO_TYPES_CODEC_PARAM)
                .ok_or(ProtocolError::MissingProperty)?,
        );
        let audio_rtp_param_objs = decode(
            audio_objs
                .get(&AUDIO_TYPES_RTP_PARAM)
                .ok_or(ProtocolError::MissingProperty)?,
        );

        let comfort_noise = audio_objs
            .get(&AUDIO_TYPES_COMFORT_NOISE)
            .ok_or(ProtocolError::MissingProperty)?
            .clone();

        let channel = audio_codec_param_objs
            .get(&AUDIO_CODEC_PARAM_TYPES_CHANNEL)
            .ok_or(ProtocolError::MissingProperty)?
            .first()
            .ok_or(ProtocolError::MissingProperty)?
            .clone();
        let bitrate = audio_codec_param_objs
            .get(&AUDIO_CODEC_PARAM_TYPES_BIT_RATE)
            .ok_or(ProtocolError::MissingProperty)?
            .first()
            .ok_or(ProtocolError::MissingProperty)?;
        let bitrate = *bitrate != 0;
        let sample_rate = audio_codec_param_objs
            .get(&AUDIO_CODEC_PARAM_TYPES_SAMPLE_RATE)
            .ok_or(ProtocolError::MissingProperty)?
            .first()
            .ok_or(ProtocolError::MissingProperty)?
            .clone();
        let sample_rate = 8 * (1 + sample_rate);
        let packet_time = audio_codec_param_objs
            .get(&AUDIO_CODEC_PARAM_TYPES_PACKET_TIME)
            .ok_or(ProtocolError::MissingProperty)?
            .first()
            .ok_or(ProtocolError::MissingProperty)?
            .clone();

        let ssrc = audio_rtp_param_objs
            .get(&RTP_PARAM_TYPES_SYNCHRONIZATION_SOURCE)
            .ok_or(ProtocolError::MissingProperty)?
            .clone();
        let ssrc = u32::from_le_bytes(ssrc.clone().try_into().map_err(|_| ProtocolError::ArrayConversion)?);
        let payload_type = audio_rtp_param_objs
            .get(&RTP_PARAM_TYPES_PAYLOAD_TYPE)
            .ok_or(ProtocolError::MissingProperty)?
            .clone();
        let max_bitrate = audio_rtp_param_objs
            .get(&RTP_PARAM_TYPES_MAX_BIT_RATE)
            .ok_or(ProtocolError::MissingProperty)?;
        let max_bitrate = u16::from_le_bytes(
            max_bitrate
                .clone()
                .try_into()
                .map_err(|_| ProtocolError::ArrayConversion)?,
        );
        let rtcp_interval = audio_rtp_param_objs
            .get(&RTP_PARAM_TYPES_RTCP_SEND_INTERVAL)
            .ok_or(ProtocolError::MissingProperty)?;
        let rtcp_interval = f32::from_le_bytes(
            rtcp_interval
                .clone()
                .try_into()
                .map_err(|_| ProtocolError::ArrayConversion)?,
        );
        let comfort_payload_type = audio_rtp_param_objs
            .get(&RTP_PARAM_TYPES_COMFORT_NOISE_PAYLOAD_TYPE)
            .ok_or(ProtocolError::MissingProperty)?
            .clone();

        info.audio = Some(StreamAudio {
            codec,
            comfort_noise,
            channel,
            bitrate,
            sample_rate,
            packet_time,
            ssrc,
            payload_type,
            max_bitrate,
            rtcp_interval,
            comfort_payload_type,
        });
    }

    Ok(info)
}

pub(crate) fn selected_stream_configuration_request(bytes: &[u8]) -> Result<SelectedStreamRequest, ProtocolError> {
    let objs = decode(bytes);
    let session_objs = decode(
        objs.get(&SELECTED_STREAM_CONFIGURATION_TYPES_SESSION)
            .ok_or(ProtocolError::MissingProperty)?,
    );

    let session_id = session_objs
        .get(&SETUP_TYPES_SESSION_ID)
        .ok_or(ProtocolError::MissingProperty)?;

    let session_id = Uuid::from_bytes(
        session_id
            .clone()
            .try_into()
            .map_err(|_| ProtocolError::ArrayConversion)?,
    );

    let request_type = session_objs
        .get(&0x02)
        .ok_or(ProtocolError::MissingProperty)?
        .first()
        .ok_or(ProtocolError::MissingProperty)?;
    let kind = match request_type {
        0 => SelectedStreamRequestKind::Stop,
        1 => SelectedStreamRequestKind::Start,
        4 => SelectedStreamRequestKind::StartAndReconfigure,
        _ => return Err(ProtocolError::MissingProperty),
    };

    Ok(SelectedStreamRequest { session_id, kind })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    pub fn stream_info() {
        let data = vec![
            1u8, 21, 2, 1, 1, 1, 16, 159, 74, 88, 188, 175, 8, 79, 248, 128, 222, 231, 61, 225, 39, 125, 40, 2, 52, 1,
            1, 0, 2, 9, 1, 1, 0, 2, 1, 0, 3, 1, 0, 3, 11, 1, 2, 128, 2, 2, 2, 104, 1, 3, 1, 30, 4, 23, 1, 1, 99, 2, 4,
            210, 68, 127, 99, 3, 2, 132, 0, 4, 4, 0, 0, 0, 63, 5, 2, 98, 5, 3, 44, 1, 1, 2, 2, 12, 1, 1, 1, 2, 1, 0, 3,
            1, 1, 4, 1, 30, 3, 22, 1, 1, 110, 2, 4, 82, 196, 23, 55, 3, 2, 24, 0, 4, 4, 0, 0, 160, 64, 6, 1, 13, 4, 1,
            0,
        ];

        let info = stream_info_response(&data).unwrap();

        assert_eq!(info.session_id.to_string(), "9f4a58bc-af08-4ff8-80de-e73de1277d28");

        let data2 = vec![
            1u8, 21, 2, 1, 1, 1, 16, 214, 2, 171, 255, 102, 170, 79, 250, 140, 43, 43, 150, 155, 231, 114, 3, 2, 52, 1,
            1, 0, 2, 9, 1, 1, 0, 2, 1, 0, 3, 1, 0, 3, 11, 1, 2, 128, 2, 2, 2, 104, 1, 3, 1, 30, 4, 23, 1, 1, 99, 2, 4,
            186, 145, 247, 16, 3, 2, 132, 0, 4, 4, 0, 0, 0, 63, 5, 2, 98, 5, 3, 44, 1, 1, 2, 2, 12, 1, 1, 1, 2, 1, 0,
            3, 1, 1, 4, 1, 30, 3, 22, 1, 1, 110, 2, 4, 115, 17, 212, 19, 3, 2, 24, 0, 4, 4, 0, 0, 160, 64, 6, 1, 13, 4,
            1, 0,
        ];

        let info = stream_info_response(&data2).unwrap();

        assert!(!info.audio.map(|audio| audio.bitrate).unwrap());
    }

    #[test]
    pub fn support_rtp() {
        let config = supported_rtp_config(true);

        assert_eq!(&config, &[0x02, 0x01, 0x00]);
    }

    #[test]
    pub fn audio_stream() {
        let config = audio_stream_config(json!({
            "codecs": [
                {
                    "type": "AAC-eld",
                    "samplerate": 16,
                }
            ]
        }));

        assert_eq!(
            &config,
            &[
                0x01, 0x0e, 0x01, 0x01, 0x02, 0x02, 0x09, 0x01, 0x01, 0x01, 0x02, 0x01, 0x00, 0x03, 0x01, 0x01, 0x02,
                0x01, 0x00
            ]
        );
    }

    #[test]
    pub fn video_stream() {
        let config = video_stream_config(json!({
            "resolutions": [
                [ 320, 180, 30 ],
                [ 320, 240, 15 ],
                [ 320, 240, 30 ],
                [ 480, 270, 30 ],
                [ 480, 360, 30 ],
                [ 640, 360, 30 ],
                [ 640, 480, 30 ],
                [ 1280, 720, 30 ],
                [ 1280, 960, 30 ],
                [ 1920, 1080, 30 ],
                [ 1600, 1200, 30 ]
          ],
          "codec": {
              "profiles": [ 0, 1, 2 ],
              "levels": [ 0, 1, 2 ]
          }
        }));

        let expected = b"\x01\xc5\x01\x01\x00\x02\x1d\x01\x01\x00\x00\x00\x01\x01\x01\x00\x00\
            \x01\x01\x02\x02\x01\x00\x00\x00\x02\x01\x01\x00\x00\x02\x01\x02\x03\x01\x00\x03\x0b\x01\x02\
            \x40\x01\x02\x02\xb4\x00\x03\x01\x1e\x00\x00\x03\x0b\x01\x02\x40\x01\x02\x02\xf0\x00\x03\x01\
            \x0f\x00\x00\x03\x0b\x01\x02\x40\x01\x02\x02\xf0\x00\x03\x01\x1e\x00\x00\x03\x0b\x01\x02\xe0\
            \x01\x02\x02\x0e\x01\x03\x01\x1e\x00\x00\x03\x0b\x01\x02\xe0\x01\x02\x02\x68\x01\x03\x01\x1e\
            \x00\x00\x03\x0b\x01\x02\x80\x02\x02\x02\x68\x01\x03\x01\x1e\x00\x00\x03\x0b\x01\x02\x80\x02\
            \x02\x02\xe0\x01\x03\x01\x1e\x00\x00\x03\x0b\x01\x02\x00\x05\x02\x02\xd0\x02\x03\x01\x1e\x00\
            \x00\x03\x0b\x01\x02\x00\x05\x02\x02\xc0\x03\x03\x01\x1e\x00\x00\x03\x0b\x01\x02\x80\x07\x02\
            \x02\x38\x04\x03\x01\x1e\x00\x00\x03\x0b\x01\x02\x40\x06\x02\x02\xb0\x04\x03\x01\x1e";

        assert_eq!(&config, expected);
    }

    #[test]
    pub fn setup_endpoint() {
        let data = vec![
            1u8, 16, 97, 204, 48, 49, 240, 116, 75, 2, 150, 143, 224, 143, 38, 45, 40, 65, 3, 25, 1, 1, 0, 2, 12, 49,
            57, 50, 46, 49, 54, 56, 46, 50, 46, 53, 50, 3, 2, 149, 192, 4, 2, 80, 238, 4, 37, 2, 16, 8, 255, 34, 115,
            19, 121, 118, 140, 129, 31, 225, 7, 8, 113, 47, 183, 3, 14, 224, 107, 26, 188, 12, 133, 251, 85, 169, 57,
            32, 164, 163, 101, 1, 1, 0, 5, 37, 2, 16, 143, 234, 217, 0, 38, 98, 183, 222, 143, 69, 27, 46, 24, 58, 169,
            193, 3, 14, 210, 110, 127, 168, 239, 73, 194, 105, 156, 195, 164, 211, 59, 231, 1, 1, 0,
        ];

        let request = setup_endpoint_request(&data).unwrap();

        assert_eq!(
            request.session_id.to_string(),
            "61cc3031-f074-4b02-968f-e08f262d2841".to_string()
        );
    }
}
