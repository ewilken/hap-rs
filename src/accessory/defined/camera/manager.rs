use std::{collections::HashMap, error::Error, net::IpAddr, sync::Arc};

use futures::{future::BoxFuture, lock::Mutex};
use serde_json::Value;
use thiserror::Error;
use uuid::Uuid;

use crate::{
    characteristic::{AsyncCharacteristicCallbacks, HapCharacteristic},
    pointer,
    server::Server,
    tlv, HapType,
};

use super::{media::MediaProvider, protocol, CameraAccessory};

use tokio::sync::mpsc::{channel, Sender};

pub struct StreamManagerBuilder<MP>
where
    MP: MediaProvider,
{
    accessory: CameraAccessory,
    provider: MP,
}

#[derive(Debug, Clone)]
pub struct SessionState {
    pub session_id: Uuid,
    pub address: IpAddr,
    pub video_port: u16,
    pub audio_port: u16,
    pub video_crypto: protocol::SetupEndpointCrypto,
    pub audio_crypto: protocol::SetupEndpointCrypto,
    pub video_ssrc: u32,
    pub audio_ssrc: u32,
}

// todo: consider types to be more restricted
#[derive(Clone, Default, Debug)]
pub struct StreamOptions {
    pub video: Value,
    pub audio: Value,
    pub srtp: bool,
}

pub struct StreamManager<MP>
where
    MP: MediaProvider,
{
    accessory: pointer::Accessory,
    inner: Arc<Mutex<StreamManagerInner<MP>>>,
}

#[derive(Debug)]
enum StreamingStatus {
    Streaming,
    Available,
}

pub struct StreamManagerInner<MP>
where
    MP: MediaProvider,
{
    streaming_status: HashMap<usize, StreamingStatus>,
    sessions: HashMap<Uuid, SessionState>,
    /// Setup endpoint change sender
    sec_sender: Sender<SetupEndpointChange>,
    /// Media provider
    provider: MP,
}

#[derive(Debug, Error)]
pub enum StreamManagerError {
    #[error("Invalid accessory")]
    InvalidAccessory,
    #[error("Missing characteristic")]
    MissingCharacteristic,
    #[error("Missing session")]
    SessionNotFound,
    #[error("Provider error: {0}")]
    Provider(Box<dyn Error + Send + Sync + 'static>),
    #[error(transparent)]
    Hap(#[from] crate::Error),
    #[error(transparent)]
    ProtocolError(#[from] protocol::ProtocolError),
}

struct SetupEndpointChange(usize, Vec<u8>);

/// Builder adds callbacks to Camera Accessory and creates
/// channel which allows modification of setup_endpoint characteristic
/// from inside a callback
impl<MP> StreamManagerBuilder<MP>
where
    MP: MediaProvider + Send + Sync + 'static,
{
    pub fn new(accessory: CameraAccessory, provider: MP) -> Self {
        Self { accessory, provider }
    }

    /// Builder is needed because in case of Camera Accessory we have to self modify value of
    /// setup_endpoints to create proper handshake with iOS device
    /// Current accessory contract doesnt allow to self modify Characteristic and locks
    /// are created when adding accessory to the server
    pub async fn build<S>(self, server: S) -> Result<StreamManager<MP>, StreamManagerError>
    where
        S: Server,
    {
        let Self {
            mut accessory,
            provider,
        } = self;

        let (sec_sender, mut sec_receiver) = channel::<SetupEndpointChange>(10);
        let mgmt = Arc::new(Mutex::new(StreamManagerInner::new(sec_sender, provider)));

        for (idx, stream) in accessory.streams.iter_mut().enumerate() {
            // set 3 characteristics responsible for streaming

            // setup_endpoint
            let m_mgmt = mgmt.clone();
            stream
                .setup_endpoint
                .on_update_async(Some(move |_: tlv::Tlv8, value: tlv::Tlv8| {
                    let mgmt = m_mgmt.clone();

                    Box::pin(async move {
                        let mut lock = mgmt.lock().await;
                        lock.setup_endpoint(idx, value).await.map_err(Box::from)
                    }) as BoxFuture<'static, std::result::Result<(), Box<dyn Error + Send + Sync>>>
                }));

            //  selected_stream_configuration
            let m_mgmt = mgmt.clone();
            stream
                .selected_stream_configuration
                .on_update_async(Some(move |_: tlv::Tlv8, value: tlv::Tlv8| {
                    let mgmt = m_mgmt.clone();

                    Box::pin(async move {
                        let mut lock = mgmt.lock().await;
                        lock.selected_stream_configuration(idx, value).await.map_err(Box::from)
                    }) as BoxFuture<'static, std::result::Result<(), Box<dyn Error + Send + Sync>>>
                }));

            // streaming_status
            let m_mgmt = mgmt.clone();
            stream.streaming_status.on_read_async(Some(move || {
                let mgmt = m_mgmt.clone();

                Box::pin(async move {
                    let mut lock = mgmt.lock().await;
                    lock.streaming_status(idx).await
                }) as BoxFuture<'static, std::result::Result<_, Box<dyn Error + Send + Sync>>>
            }));

            // set stream configuration options and defaults
            let lock = mgmt.lock().await;
            let options = lock
                .provider()
                .options(idx)
                .map_err(|err| StreamManagerError::Provider(Box::new(err)))?
                .clone();
            log::info!("settings stream options ({idx}: {:?}", options);

            stream
                .supported_video_stream_configuration
                .set_value(protocol::video_stream_config(options.video).into())
                .await?;
            stream
                .supported_audio_stream_configuration
                .set_value(protocol::audio_stream_config(options.audio).into())
                .await?;
            stream
                .supported_rtp_configuration
                .set_value(protocol::supported_rtp_config(options.srtp).into())
                .await?;
            stream
                .streaming_status
                .set_value(tlv::encode(vec![(0x01u8, vec![protocol::STREAMING_STATUS_AVAILABLE])]).into())
                .await?;
            if let Some(ref mut active) = stream.active {
                active.set_value(1.into()).await?;
            }
        }

        // add accessory and create Arc<Mutex<_>>
        let accessory = server.add_accessory(accessory).await?;

        // create channel which will postpone moditication of setup_endpoint
        // so we won't encounter any deadlocks
        let acc = accessory.clone();
        tokio::spawn(async move {
            while let Some(SetupEndpointChange(idx, data)) = sec_receiver.recv().await {
                let mut acc = acc.lock().await;
                let mut services = acc
                    .get_mut_services()
                    .into_iter()
                    .filter(|s| s.get_type() == HapType::CameraStreamManagement)
                    .collect::<Vec<_>>();

                if let Some(service) = services.get_mut(idx) {
                    if let Err(err) = service
                        .get_mut_characteristic(HapType::SetupEndpoint)
                        .unwrap()
                        .set_value(data.into())
                        .await
                    {
                        log::error!("cannot set setup_endpoint characteristic for stream: {idx}, {err}")
                    }
                }
            }
        });

        Ok(StreamManager::new(accessory, mgmt))
    }
}

impl<MP> StreamManagerInner<MP>
where
    MP: MediaProvider,
{
    fn new(sec_sender: Sender<SetupEndpointChange>, provider: MP) -> Self {
        Self {
            sec_sender,
            provider,
            sessions: Default::default(),
            streaming_status: Default::default(),
        }
    }

    fn provider(&self) -> &MP {
        &self.provider
    }

    pub async fn setup_endpoint(
        &mut self,
        idx: usize,
        value: tlv::Tlv8,
    ) -> std::result::Result<(), StreamManagerError> {
        log::info!("setup_endpoint({idx}): set");
        log::trace!("setup_endpoint({idx}): {:?}", value);

        let req = protocol::setup_endpoint_request(value.0.as_ref())?;
        log::trace!("setup_endpoint({idx}): request received: {req:?}");

        if self.sessions.contains_key(&req.session_id) {
            // prevent taking action after self change
            return Ok(());
        }

        let res = protocol::setup_endpoint_response(
            &req,
            self.provider
                .address(idx)
                .map_err(|err| StreamManagerError::Provider(Box::new(err)))?,
        );
        log::info!("setup_endpoint({idx}): creating session: {:?}", req.session_id);

        self.sessions.insert(
            req.session_id,
            SessionState {
                session_id: req.session_id,
                address: req.ip,
                video_port: req.video_port,
                audio_port: req.audio_port,
                video_crypto: req.video_crypto,
                audio_crypto: req.audio_crypto,
                video_ssrc: res.video_ssrc,
                audio_ssrc: res.audio_ssrc,
            },
        );

        if let Err(err) = self.sec_sender.send(SetupEndpointChange(idx, res.tlv)).await {
            log::error!("cannot respond to setup_endpoint request: {err:?}");
        }

        Ok(())
    }

    pub async fn selected_stream_configuration(
        &mut self,
        idx: usize,
        value: tlv::Tlv8,
    ) -> std::result::Result<(), StreamManagerError> {
        log::info!("selected_stream_configuration({idx}): set");
        log::debug!("selected_stream_configuration({idx}): {:?}", value);

        let req = protocol::selected_stream_configuration_request(&value.0)?;
        let session = self
            .sessions
            .get(&req.session_id)
            .ok_or(StreamManagerError::SessionNotFound)?
            .clone();
        log::info!("selected_stream_configuration({idx}): request: {:?}", req.session_id);
        log::debug!("selected_stream_configuration({idx}): request: {req:?}, session: {session:?}");

        match req.kind {
            protocol::SelectedStreamRequestKind::Start => {
                log::info!("starting stream: {idx}");

                let data = protocol::stream_info_response(&value.0)?;

                self.provider.start(idx, data, &session).await.map_err(|err| {
                    self.streaming_status.insert(idx, StreamingStatus::Available);
                    StreamManagerError::Provider(Box::new(err))
                })?;

                self.streaming_status.insert(idx, StreamingStatus::Streaming);
            },
            protocol::SelectedStreamRequestKind::Stop => {
                log::info!("stopping stream: {idx}");

                self.provider
                    .stop(idx, &session)
                    .await
                    .map_err(|err| StreamManagerError::Provider(Box::new(err)))?;

                self.streaming_status.insert(idx, StreamingStatus::Available);
            },
            protocol::SelectedStreamRequestKind::StartAndReconfigure => {
                log::info!("starting and reconfiguring stream: {idx}");

                let data = protocol::stream_info_response(&value.0)?;
                self.provider.reconfigure(idx, data, &session).await.map_err(|err| {
                    self.streaming_status.insert(idx, StreamingStatus::Available);
                    StreamManagerError::Provider(Box::new(err))
                })?;

                self.streaming_status.insert(idx, StreamingStatus::Streaming);
            },
        }

        Ok(())
    }

    pub async fn streaming_status(
        &mut self,
        idx: usize,
    ) -> std::result::Result<Option<tlv::Tlv8>, Box<dyn Error + Send + Sync>> {
        let status = self.streaming_status.get(&idx);

        log::info!("streaming_status({idx}): read: {:?}", status);
        log::debug!("streaming_status({idx}): read");

        Ok(Some(tlv::Tlv8(tlv::encode(vec![(
            0x01u8,
            vec![match status {
                Some(StreamingStatus::Available) => protocol::STREAMING_STATUS_AVAILABLE,
                Some(StreamingStatus::Streaming) => protocol::STREAMING_STATUS_STREAMING,
                None => protocol::STREAMING_STATUS_AVAILABLE,
            }],
        )]))))
    }
}

impl<MP> StreamManager<MP>
where
    MP: MediaProvider,
{
    pub fn new(accessory: pointer::Accessory, inner: Arc<Mutex<StreamManagerInner<MP>>>) -> Self {
        Self { accessory, inner }
    }

    pub fn accessory(&self) -> pointer::Accessory {
        self.accessory.clone()
    }

    pub async fn sessions(&self) -> HashMap<Uuid, SessionState> {
        let inner = self.inner.lock().await;
        inner.sessions.clone()
    }
}
