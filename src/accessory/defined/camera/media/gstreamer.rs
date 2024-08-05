use std::{collections::HashMap, net::IpAddr, sync::Arc};

use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

use crate::accessory::camera::{
    manager::{SessionState, StreamOptions},
    protocol,
};

use super::MediaProvider;
use tokio::{
    sync::{oneshot, Mutex},
    task::JoinHandle,
};

use gst::prelude::*;

#[cfg(feature = "camera-gstreamer")]
use gstreamer as gst;

#[cfg(not(feature = "camera-gstreamer"))]
use gstreamer18 as gst;

#[derive(Clone)]
pub struct StreamConfig {
    pub address: IpAddr,
    pub pipeline: String,
    pub options: StreamOptions,
}

#[derive(Error, Debug)]
pub enum GstreamerError {
    #[error("stream not found")]
    StreamNotFound,
    #[error("cannot acquire lock")]
    CannotAcquireLock,
    #[error("cannot restart session")]
    CannotRestartSession,
    #[error("cannot find required property")]
    MissingProperty,
    #[error(transparent)]
    Glib(#[from] gst::glib::Error),
}

#[derive(Clone)]
pub struct Gstreamer {
    configs: Vec<StreamConfig>,
    streams: Arc<Mutex<HashMap<Uuid, HashMap<usize, StreamState>>>>,
}

pub struct StreamState {
    _handle: JoinHandle<()>,
    close: oneshot::Sender<oneshot::Sender<()>>,
}

impl Gstreamer {
    pub fn new(configs: Vec<StreamConfig>) -> Result<Self, GstreamerError> {
        gst::init()?;

        Ok(Self {
            configs,
            streams: Default::default(),
        })
    }
}

impl Gstreamer {
    fn pipeline(
        &self,
        idx: usize,
        stream: protocol::StreamInfo,
        session: &SessionState,
    ) -> Result<String, GstreamerError> {
        // key
        let mut both = session.video_crypto.master_key.clone();
        both.append(&mut session.video_crypto.master_salt.clone());
        let key = hex::encode(&both);

        let ssrc = session.video_ssrc;
        let host = session.address.to_string();
        let port = session.video_port;

        let max_bitrate = stream
            .video_rtp
            .and_then(|rtp| rtp.max_bitrate)
            .ok_or(GstreamerError::MissingProperty)?;
        let width = stream
            .video_attributes
            .as_ref()
            .map(|va| va.width)
            .ok_or(GstreamerError::MissingProperty)?;
        let height = stream
            .video_attributes
            .as_ref()
            .map(|va| va.height)
            .ok_or(GstreamerError::MissingProperty)?;
        let fps = stream
            .video_attributes
            .as_ref()
            .map(|va| va.fps)
            .ok_or(GstreamerError::MissingProperty)?;

        let StreamConfig { pipeline, .. } = self.configs.get(idx).ok_or(GstreamerError::StreamNotFound)?;

        log::info!("video pipeline created: width: {width}, height: {height}, fps: {fps}, max_bitrate: {max_bitrate}");

        Ok(format!(
            "{pipeline} \
            ! videorate \
            ! video/x-raw,framerate={fps}/1 \
            ! videoconvert \
            ! videoscale ! video/x-raw,width={width},height={height} \
            ! x264enc tune=zerolatency bitrate={max_bitrate} \
            ! video/x-h264,profile=baseline \
            ! rtph264pay config-interval=1 pt=99 ssrc={ssrc} \
            ! srtpenc key={key} \
            ! udpsink host={host} port={port} async=false"
        ))
    }
}

#[async_trait]
impl MediaProvider for Gstreamer {
    type Error = GstreamerError;

    fn address(&self, idx: usize) -> Result<&std::net::IpAddr, GstreamerError> {
        self.configs
            .get(idx)
            .map(|c| &c.address)
            .ok_or(GstreamerError::StreamNotFound)
    }

    fn options(&self, idx: usize) -> Result<&StreamOptions, Self::Error> {
        self.configs
            .get(idx)
            .map(|c| &c.options)
            .ok_or(GstreamerError::StreamNotFound)
    }

    async fn start(&self, idx: usize, stream: protocol::StreamInfo, session: &SessionState) -> Result<(), Self::Error> {
        let mut streams = self.streams.lock().await;

        let current = streams.entry(stream.session_id).or_default();
        if current.contains_key(&idx) {
            return Err(GstreamerError::CannotRestartSession);
        }

        let pipeline = self.pipeline(idx, stream, session)?;
        let (close, mut closed) = oneshot::channel::<oneshot::Sender<()>>();
        current.insert(
            idx,
            StreamState {
                close,
                _handle: tokio::task::spawn_blocking(move || {
                    log::info!("creating pipeline");

                    log::debug!("gstreamer pipeline: {pipeline}");

                    #[cfg(feature = "camera-gstreamer")]
                    let pipeline = gst::parse::launch(&pipeline).expect("cannot build gstreamer pipeline");
                    #[cfg(not(feature = "camera-gstreamer"))]
                    let pipeline = gst::parse_launch(&pipeline).expect("cannot build gstreamer pipeline");

                    pipeline
                        .set_state(gst::State::Playing)
                        .expect("cannost start streamer pipeline");

                    log::info!("pipeline playing");

                    let mut confirmation: Option<oneshot::Sender<()>> = None;

                    let bus = pipeline.bus().unwrap();
                    for msg in bus.iter_timed(None) {
                        if let Ok(confirm) = closed.try_recv() {
                            log::info!("closing gstreamer pipeline");
                            confirmation = Some(confirm);
                            break;
                        }

                        match msg.view() {
                            gst::MessageView::Eos(_) => break,
                            gst::MessageView::Error(err) => log::error!("{:?}", err),
                            _ => (),
                        }
                    }

                    // Clean up
                    pipeline
                        .set_state(gst::State::Null)
                        .expect("cannot clean up gstreamer pipeline");

                    log::info!("gstreamer pipeline finished");

                    if let Some(confirm) = confirmation {
                        let _ = confirm.send(());
                    }
                }),
            },
        );

        Ok(())
    }

    async fn stop(&self, idx: usize, session: &SessionState) -> Result<(), Self::Error> {
        let mut streams = self.streams.lock().await;

        let current = streams.entry(session.session_id).or_default();

        if let Some(stream) = current.remove(&idx) {
            let (confirm, confirmed) = oneshot::channel::<()>();
            if stream.close.send(confirm).is_ok() {
                // during reconfiguration we have to wait for pipeline to stop
                let _ = confirmed.await;
            }
        }

        Ok(())
    }

    async fn reconfigure(
        &self,
        idx: usize,
        stream: protocol::StreamInfo,
        session: &SessionState,
    ) -> Result<(), Self::Error> {
        self.stop(idx, session).await?;
        self.start(idx, stream, session).await
    }
}
