use std::time::Duration;

use futures::future::Future;
use log::debug;
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

    // this should be the correct implementation, but (as of 0.4.1) the UDP stream implementation of libmdns is
    // broken. instead of polling & waking correctly, the stream is busy looping on Poll::Pending and needs to be
    // put to a thread with a distinct runtime that can be put to sleep periodically. urgh. TODO: fix libmdns
    // /// Returns a Future handle to the mDNS responder operation that can be passed to an executor.
    // pub fn run_handle(&self) -> impl Future<Output = ()> + Send + '_ {
    //     let config = self.config.clone();
    //     let (responder, responder_task) = libmdns::Responder::with_default_handle().expect(
    //         "creating mDNS
    //     responder",
    //     );
    //     let register_task = async move {
    //         loop {
    //             let config = config.lock().await;
    //
    //             let name = config.name.clone();
    //             let port = config.socket_addr.port();
    //             let tr = config.txt_records();
    //             let status_flag = config.status_flag;
    //
    //             drop(config);
    //
    //             let service = responder.register("_hap._tcp".into(), name, port, &[
    //                 &tr[0], &tr[1], &tr[2], &tr[3], &tr[4], &tr[5], &tr[6], &tr[7],
    //             ]);
    //             debug!("announcing mDNS: {:?}", &tr);
    //
    //             drop(service);
    //
    //             time::delay_for(Duration::from_millis(match status_flag {
    //                 crate::transport::bonjour::BonjourStatusFlag::NotPaired => 1000,
    //                 _ => 20_000,
    //             }))
    //             .await;
    //         }
    //     };
    //
    //     let responder_handle = tokio::spawn(responder_task);
    //
    //     future::join(responder_handle, register_task).map(|_| ())
    // }

    /// Returns a Future handle to the mDNS responder operation that can be passed to an executor.
    pub fn run_handle(&self) -> impl Future<Output = ()> + Send + '_ {
        let config = self.config.clone();
        std::thread::spawn(move || {
            let mut rt = tokio::runtime::Runtime::new().expect("creating tokio runtime");
            rt.block_on(async move {
                let responder = libmdns::Responder::new().expect("couldn't create mDNS responder");

                loop {
                    let config = config.lock().await;

                    let name = config.name.clone();
                    let port = config.socket_addr.port();
                    let tr = config.txt_records();
                    let status_flag = config.status_flag;

                    drop(config);

                    let name = name.clone();

                    let _svc = responder.register("_hap._tcp".into(), name, port, &[
                        &tr[0], &tr[1], &tr[2], &tr[3], &tr[4], &tr[5], &tr[6], &tr[7],
                    ]);
                    debug!("announcing mDNS: {:?}", &tr);

                    time::delay_for(Duration::from_millis(match status_flag {
                        crate::transport::bonjour::BonjourStatusFlag::NotPaired => 1000,
                        _ => 20_000,
                    }))
                    .await;
                }

                Ok(()) as Result<(), ()>
            })
            .expect("starting runtime");
        });

        futures::future::ready(())
    }
}
