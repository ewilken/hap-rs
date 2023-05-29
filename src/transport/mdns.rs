
cfg_if::cfg_if! {
    if #[cfg(libmdns)] {
        use libmdns::{Responder, Service};
    } else {
        type Responder = ();
        type Service= ();
    }
}

use log::debug;

use crate::pointer;

/// An mDNS Responder. Used to announce the Accessory's name and HAP TXT records to potential controllers.
pub struct MdnsResponder {
    config: pointer::Config,
    responder: Responder,
    service: Option<Service>,
    task: Option<Box<dyn futures::Future<Output = ()> + Unpin + std::marker::Send>>,
}

impl MdnsResponder {
    /// Creates a new mDNS Responder.
    pub async fn new(config: pointer::Config) -> Self {
        cfg_if::cfg_if! {
            if #[cfg(libmdns)] {
                let (responder, task) = libmdns::Responder::with_default_handle().expect("creating mDNS responder");
            } else {
                let (responder, task) = todo!();
            }
        }


        MdnsResponder {
            config,
            responder,
            service: None,
            task: Some(task),
        }
    }

    /// Derives new mDNS TXT records from the server's `Config`.
    pub async fn update_records(&mut self) {
        debug!("attempting to set mDNS records");

        self.service = None;

        let c = self.config.lock().await;

        let name = c.name.clone();
        let port = c.port;
        let tr = c.txt_records();

        drop(c);

        todo!();
        /*
        self.service = Some(self.responder.register("_hap._tcp".into(), name, port, &[
            &tr[0], &tr[1], &tr[2], &tr[3], &tr[4], &tr[5], &tr[6], &tr[7],
        ]));
        */

        debug!("setting mDNS records: {:?}", &tr);
    }

    /// Returns the mDNS task to throw on a scheduler.
    pub fn run_handle(&mut self) -> Box<dyn futures::Future<Output = ()> + Unpin + std::marker::Send> {
        match self.task.take() {
            Some(task) => task,
            // if the task handle is gone, recreate the whole responder
            None => {
                cfg_if::cfg_if! {
                    if #[cfg(libmdns)] {
                        let (responder, task) = libmdns::Responder::with_default_handle().expect("creating mDNS responder");

                    } else {
                        let (responder, task) = todo!();
                    }
                }
                self.responder = responder;

                task
            },
        }
    }
}
