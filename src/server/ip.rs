use async_trait::async_trait;
use futures::{
    future::{BoxFuture, FutureExt},
    lock::Mutex,
};
use log::info;
use std::sync::Arc;

use crate::{
    accessory::HapAccessory,
    config::Config,
    event::{Event, EventEmitter},
    pointer,
    server::{IdentifierCache, Server},
    storage::{accessory_list::AccessoryList, Storage},
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
    identifier_cache: IdentifierCache,
}

impl IpServer {
    /// Creates a new `IpServer`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hap::{
    ///     accessory::{lightbulb::LightbulbAccessory, AccessoryCategory, AccessoryInformation},
    ///     server::{IpServer, Server},
    ///     storage::{FileStorage, Storage},
    ///     tokio,
    ///     Config,
    ///     MacAddress,
    ///     Pin,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let lightbulb = LightbulbAccessory::new(1, AccessoryInformation {
    ///         name: "Acme Lightbulb".into(),
    ///         ..Default::default()
    ///     })
    ///     .unwrap();
    ///
    ///     let mut storage = FileStorage::current_dir().await.unwrap();
    ///
    ///     let config = match storage.load_config().await {
    ///         Ok(config) => config,
    ///         Err(_) => {
    ///             let config = Config {
    ///                 pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap(),
    ///                 name: "Acme Lightbulb".into(),
    ///                 device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
    ///                 category: AccessoryCategory::Lightbulb,
    ///                 ..Default::default()
    ///             };
    ///             storage.save_config(&config).await.unwrap();
    ///             config
    ///         },
    ///     };
    ///
    ///     let mut server = IpServer::new(config, storage).unwrap();
    ///     server.add_accessory(lightbulb).await.unwrap();
    ///
    ///     let handle = server.run_handle();
    ///
    ///     std::env::set_var("RUST_LOG", "hap=info");
    ///     env_logger::init();
    ///
    ///     //handle.await;
    /// }
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

        let identifier_cache = IdentifierCache::new();

        let server = IpServer {
            config,
            storage,
            accessory_list,
            event_emitter,
            http_server,
            mdns_responder,
            identifier_cache,
        };

        Ok(server)
    }
}

#[async_trait]
impl Server for IpServer {
    fn run_handle(&self) -> BoxFuture<Result<()>> {
        let http_handle = self.http_server.run_handle();
        let mdns_handle = self.mdns_responder.run_handle();

        let handle = async {
            futures::try_join!(http_handle, mdns_handle.map(|_| Ok(())))?;

            Ok(())
        }
        .boxed();

        Box::pin(handle)
    }

    fn config_pointer(&self) -> pointer::Config { self.config.clone() }

    fn storage_pointer(&self) -> pointer::Storage { self.storage.clone() }

    async fn add_accessory<A: HapAccessory + 'static>(&self, accessory: A) -> Result<pointer::Accessory> {
        let accessory = self.accessory_list.lock().await.add_accessory(Box::new(accessory))?;

        let mut config = self.config.lock().await;
        config.configuration_number += 1;
        self.storage.lock().await.save_config(&config).await?;

        Ok(accessory)
    }

    async fn remove_accessory(&self, accessory: &pointer::Accessory) -> Result<()> {
        self.accessory_list.lock().await.remove_accessory(&accessory).await?;

        let mut config = self.config.lock().await;
        config.configuration_number += 1;

        Ok(())
    }
}
