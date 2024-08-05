use std::{error::Error, net::IpAddr};

use async_trait::async_trait;

use super::{
    manager::{SessionState, StreamOptions},
    protocol,
};

#[cfg(any(feature = "camera-gstreamer", feature = "camera-gstreamer18"))]
pub mod gstreamer;

#[async_trait]
pub trait MediaProvider {
    type Error: Error + Send + Sync + 'static;

    /// Get address of source device sending video stream
    fn address(&self, idx: usize) -> Result<&IpAddr, Self::Error>;

    /// Get available format for specified stream
    fn options(&self, idx: usize) -> Result<&StreamOptions, Self::Error>;

    /// Start video stream
    async fn start(&self, idx: usize, stream: protocol::StreamInfo, session: &SessionState) -> Result<(), Self::Error>;

    /// Reconfigure video stream
    async fn reconfigure(
        &self,
        idx: usize,
        stream: protocol::StreamInfo,
        session: &SessionState,
    ) -> Result<(), Self::Error>;

    /// Stop video stream
    async fn stop(&self, idx: usize, session: &SessionState) -> Result<(), Self::Error>;
}
