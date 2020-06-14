use std::sync::Arc;

use async_trait::async_trait;
use futures::{
    future::{self, BoxFuture, FutureExt},
    lock::Mutex,
};
use log::info;

use crate::{
    config::Config,
    event::{Event, EventEmitter},
    pointer,
    server::Server,
    storage::{
        accessory_list::{AccessoryList, AccessoryListMember},
        Storage,
    },
    transport::{http::server::Server as HttpServer, mdns::MdnsResponder},
    BonjourStatusFlag,
    Result,
};

/// HAP Server via TCP/IP.
#[derive(Clone)]
pub struct IpServer {
    config: pointer::Config,
    storage: pointer::Storage,
    accessory_list: pointer::AccessoryList,
    event_emitter: pointer::EventEmitter,
    http_server: HttpServer,
    mdns_responder: MdnsResponder,
}

impl IpServer {
    /// Creates a new `IpTransport`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hap::{
    ///     accessory::{bridge, lightbulb, Category, Information},
    ///     transport::{IpTransport, Transport},
    ///     Config,
    /// };
    ///
    /// let config = Config {
    ///     pin: "11122333".into(),
    ///     name: "Acme Lighting".into(),
    ///     category: Category::Bridge,
    ///     ..Default::default()
    /// };
    ///
    /// let bridge_info = Information {
    ///     name: "Bridge".into(),
    ///     ..Default::default()
    /// };
    /// let first_bulb_info = Information {
    ///     name: "Bulb 1".into(),
    ///     ..Default::default()
    /// };
    /// let second_bulb_info = Information {
    ///     name: "Bulb 2".into(),
    ///     ..Default::default()
    /// };
    ///
    /// let bridge = bridge::new(bridge_info).unwrap();
    /// let first_bulb = lightbulb::new(first_bulb_info).unwrap();
    /// let second_bulb = lightbulb::new(second_bulb_info).unwrap();
    ///
    /// let mut ip_transport = IpTransport::new(config).unwrap();
    /// ip_transport.add_accessory(bridge).unwrap();
    /// ip_transport.add_accessory(first_bulb).unwrap();
    /// ip_transport.add_accessory(second_bulb).unwrap();
    ///
    /// //ip_transport.start().unwrap();
    /// ```
    pub fn new<S: Storage + Send + Sync + 'static>(config: Config, storage: S) -> Result<Self> {
        let config = Arc::new(Mutex::new(config));
        let storage: pointer::Storage = Arc::new(Mutex::new(Box::new(storage)));

        let config_ = config.clone();
        let storage_ = storage.clone();
        let mut event_emitter = EventEmitter::new();

        // TODO: count pairings & override `config.status_flag`

        event_emitter.add_listener(Box::new(move |event| {
            let config_ = config_.clone();
            let storage_ = storage_.clone();
            async move {
                match *event {
                    Event::ControllerPaired { id } => {
                        info!("controller {} paired", id);

                        if let Ok(count) = storage_.lock().await.count_pairings().await {
                            if count > 0 {
                                info!("1 or more controllers paired; setting Bonjour status flag to `Zero`");

                                let mut c = config_.lock().await;
                                c.status_flag = BonjourStatusFlag::Zero;
                            }
                        }
                    },
                    Event::ControllerUnpaired { id } => {
                        info!("controller {} unpaired", id);

                        if let Ok(count) = storage_.lock().await.count_pairings().await {
                            if count == 0 {
                                info!("0 controllers paired; setting Bonjour status flag to `Not Paired`");

                                let mut c = config_.lock().await;
                                c.status_flag = BonjourStatusFlag::NotPaired;
                            }
                        }
                    },
                    _ => {},
                }
            }
            .boxed()
        }));

        let event_emitter = Arc::new(Mutex::new(event_emitter));
        let accessory_list = Arc::new(Mutex::new(AccessoryList::new(event_emitter.clone())));

        let http_server = HttpServer::new(
            config.clone(),
            storage.clone(),
            accessory_list.clone(),
            event_emitter.clone(),
        );
        let mdns_responder = MdnsResponder::new(config.clone());

        let server = IpServer {
            config,
            storage,
            accessory_list,
            event_emitter,
            http_server,
            mdns_responder,
        };

        Ok(server)
    }
}

#[async_trait]
impl Server for IpServer {
    fn run_handle(&self) -> BoxFuture<()> {
        let http_handle = self.http_server.run_handle();
        let mdns_handle = self.mdns_responder.run_handle();

        Box::pin(future::join(http_handle, mdns_handle).map(|_| ()).boxed())
    }

    fn config_pointer(&self) -> pointer::Config { self.config.clone() }

    fn storage_pointer(&self) -> pointer::Storage { self.storage.clone() }

    async fn add_accessory<A: 'static + AccessoryListMember + Send + Sync>(
        &mut self,
        accessory: A,
    ) -> Result<pointer::AccessoryListMember> {
        let accessory = self
            .accessory_list
            .lock()
            .await
            .add_accessory(Box::new(accessory))
            .await?;

        let mut config = self.config.lock().await;
        config.configuration_number += 1;

        Ok(accessory)
    }

    async fn remove_accessory(&mut self, accessory: &pointer::AccessoryListMember) -> Result<()> {
        self.accessory_list.lock().await.remove_accessory(&accessory).await?;

        let mut config = self.config.lock().await;
        config.configuration_number += 1;

        Ok(())
    }
}
