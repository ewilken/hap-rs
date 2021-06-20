use futures::{
    future::{self, Future},
    FutureExt,
};
use log::debug;
use std::time::Duration;
use tokio::time;

use crate::pointer;

/// An mDNS Responder. Used to announce the Accessory's name and HAP TXT records to potential controllers.
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
        let (responder, responder_task) = libmdns::Responder::with_default_handle().expect("creating mDNS responder");
        let register_task = async move {
            loop {
                let config = config.lock().await;

                let name = config.name.clone();
                let port = config.port;
                let tr = config.txt_records();
                let status_flag = config.status_flag;

                drop(config);

                let service = responder.register("_hap._tcp".into(), name, port, &[
                    &tr[0], &tr[1], &tr[2], &tr[3], &tr[4], &tr[5], &tr[6], &tr[7],
                ]);
                debug!("announcing mDNS: {:?}", &tr);

                drop(service);

                time::sleep(Duration::from_millis(match status_flag {
                    crate::transport::bonjour::BonjourStatusFlag::NotPaired => 500,
                    _ => 20_000,
                }))
                .await;
            }
        };

        let responder_handle = tokio::spawn(responder_task);

        future::join(responder_handle, register_task).map(|_| ())
    }
}
