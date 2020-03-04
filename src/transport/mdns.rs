use std::time::Duration;

use futures::{future::Future, stream::StreamExt};
use log::debug;
use tokio::time;

use crate::pointer;

/// An mDNS Responder. Used to announce the Accessory's name and HAP TXT records to potential
/// controllers.
#[derive(Debug, Clone)]
pub struct MdnsResponder {
    config: pointer::Config,
}

impl MdnsResponder {
    /// Creates a new mDNS Responder.
    pub fn new(config: pointer::Config) -> Self { MdnsResponder { config } }

    /// Returns a Future handle to the mDNS responder operation that can be passed to an executor.
    pub fn run_handle(&self) -> impl Future<Output = ()> + Send + '_ {
        let config = self.config.clone();
        async move {
            let responder = libmdns::Responder::new().expect("couldn't create mDNS responder");
            let mut interval = time::interval(Duration::from_millis(200));
            while let Some(_) = interval.next().await {
                let config = config.lock().expect("couldn't access config");
                let name = config.name.clone();
                let port = config.socket_addr.port();
                let tr = config.txt_records();
                let _svc = responder.register("_hap._tcp".into(), name, port, &[
                    &tr[0], &tr[1], &tr[2], &tr[3], &tr[4], &tr[5], &tr[6], &tr[7],
                ]);
                debug!("announcing mDNS: {:?}", &tr);
            }
        }
    }
}
