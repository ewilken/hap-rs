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
    server::Server,
    storage::{accessory_database::AccessoryDatabase, Storage},
    transport::{http::server::Server as HttpServer, mdns::MdnsResponder},
    BonjourStatusFlag,
    Result,
};

/// HAP Server via TCP/IP.
#[derive(Clone)]
pub struct IpServer {
    config: pointer::Config,
    storage: pointer::Storage,
    accessory_database: pointer::AccessoryDatabase,
    event_emitter: pointer::EventEmitter,
    http_server: HttpServer,
    mdns_responder: pointer::MdnsResponder,
    aid_cache: Arc<Mutex<Vec<u64>>>,
}

impl IpServer {
    /// Creates a new `IpServer`.
    ///
    /// # Examples
    /// ```no_run
    /// use hap::{
    ///     accessory::{lightbulb::LightbulbAccessory, AccessoryCategory, AccessoryInformation},
    ///     server::{IpServer, Server},
    ///     storage::{FileStorage, Storage},
    ///     tokio,
    ///     Config,
    ///     MacAddress,
    ///     Pin,
    ///     Result,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let lightbulb = LightbulbAccessory::new(1, AccessoryInformation {
    ///         name: "Acme Lightbulb".into(),
    ///         ..Default::default()
    ///     })?;
    ///
    ///     let mut storage = FileStorage::current_dir().await?;
    ///
    ///     let config = match storage.load_config().await {
    ///         Ok(mut config) => {
    ///             config.redetermine_local_ip();
    ///             storage.save_config(&config).await?;
    ///             config
    ///         },
    ///         Err(_) => {
    ///             let config = Config {
    ///                 pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3])?,
    ///                 name: "Acme Lightbulb".into(),
    ///                 device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
    ///                 category: AccessoryCategory::Lightbulb,
    ///                 ..Default::default()
    ///             };
    ///             storage.save_config(&config).await?;
    ///             config
    ///         },
    ///     };
    ///
    ///     let mut server = IpServer::new(config, storage).await?;
    ///     server.add_accessory(lightbulb).await?;
    ///
    ///     let handle = server.run_handle();
    ///
    ///     std::env::set_var("RUST_LOG", "hap=info");
    ///     env_logger::init();
    ///
    ///     handle.await
    /// }
    /// ```
    pub async fn new<S: Storage + Send + Sync + 'static>(config: Config, storage: S) -> Result<Self> {
        let config = Arc::new(Mutex::new(config));
        let storage: pointer::Storage = Arc::new(Mutex::new(Box::new(storage)));

        let config_ = config.clone();
        let storage_ = storage.clone();
        let mut event_emitter = EventEmitter::new();

        if storage_.lock().await.count_pairings().await? > 0 {
            info!("1 or more controllers paired; setting Bonjour status flag to `Zero`");

            let mut c = config_.lock().await;
            c.status_flag = BonjourStatusFlag::Zero;

            storage_.lock().await.save_config(&c).await?;
        } else {
            info!("0 controllers paired; setting Bonjour status flag to `Not Paired`");

            let mut c = config_.lock().await;
            c.status_flag = BonjourStatusFlag::NotPaired;

            storage_.lock().await.save_config(&c).await?;
        }

        let mdns_responder = Arc::new(Mutex::new(MdnsResponder::new(config.clone()).await));
        let mdns_responder_ = mdns_responder.clone();

        event_emitter.add_listener(Box::new(move |event| {
            // let config_ = config_.clone();
            let storage_ = storage_.clone();
            let mdns_responder_ = mdns_responder_.clone();
            async move {
                match *event {
                    Event::ControllerPaired { id } => {
                        info!("controller {} paired", id);

                        if let Ok(count) = storage_.lock().await.count_pairings().await {
                            if count > 0 {
                                info!("1 or more controllers paired; setting Bonjour status flag to `Zero`");

                                // TODO - this deadlocks
                                // let mut c = config_.lock().await;
                                // c.status_flag = BonjourStatusFlag::Zero;

                                // storage_
                                //     .lock()
                                //     .await
                                //     .save_config(&c)
                                //     .await
                                //     .map_err(|e| error!("error saving the config: {:?}", e))
                                //     .ok();

                                // drop(c);

                                mdns_responder_.lock().await.update_records().await;
                            }
                        }
                    },
                    Event::ControllerUnpaired { id } => {
                        info!("controller {} unpaired", id);

                        if let Ok(count) = storage_.lock().await.count_pairings().await {
                            if count == 0 {
                                info!("0 controllers paired; setting Bonjour status flag to `Not Paired`");

                                // TODO - this deadlocks
                                // let mut c = config_.lock().await;
                                // c.status_flag = BonjourStatusFlag::NotPaired;

                                // storage_
                                //     .lock()
                                //     .await
                                //     .save_config(&c)
                                //     .await
                                //     .map_err(|e| error!("error saving the config: {:?}", e))
                                //     .ok();

                                // drop(c);

                                mdns_responder_.lock().await.update_records().await;
                            }
                        }
                    },
                    _ => {},
                }
            }
            .boxed()
        }));

        let event_emitter = Arc::new(Mutex::new(event_emitter));
        let accessory_database = Arc::new(Mutex::new(AccessoryDatabase::new(event_emitter.clone())));

        let http_server = HttpServer::new(
            config.clone(),
            storage.clone(),
            accessory_database.clone(),
            event_emitter.clone(),
            mdns_responder.clone(),
        );

        let mut storage_lock = storage.lock().await;
        let aid_cache = Arc::new(Mutex::new(match storage_lock.load_aid_cache().await {
            Ok(aid_cache) => aid_cache,
            Err(_) => {
                storage_lock.delete_aid_cache().await.ok();
                let aid_cache = Vec::new();
                storage_lock.save_aid_cache(&aid_cache).await?;
                aid_cache
            },
        }));
        drop(storage_lock);

        let server = IpServer {
            config,
            storage,
            accessory_database,
            event_emitter,
            http_server,
            mdns_responder,
            aid_cache,
        };

        Ok(server)
    }
}

#[async_trait]
impl Server for IpServer {
    fn run_handle(&self) -> BoxFuture<Result<()>> {
        let http_handle = self.http_server.run_handle();
        let mdns_responder = self.mdns_responder.clone();

        let handle = async move {
            let mdns_handle = mdns_responder.lock().await.run_handle();

            futures::try_join!(http_handle, mdns_handle.map(|_| Ok(())))?;

            Ok(())
        }
        .boxed();

        Box::pin(handle)
    }

    fn config_pointer(&self) -> pointer::Config { self.config.clone() }

    fn storage_pointer(&self) -> pointer::Storage { self.storage.clone() }

    async fn add_accessory<A: HapAccessory + 'static>(&self, accessory: A) -> Result<pointer::Accessory> {
        let aid = accessory.get_id();

        let accessory = self
            .accessory_database
            .lock()
            .await
            .add_accessory(Box::new(accessory))?;

        let mut aid_cache = self.aid_cache.lock().await;
        if !aid_cache.contains(&aid) {
            aid_cache.push(aid);
            self.storage.lock().await.save_aid_cache(&aid_cache).await?;

            let mut config = self.config.lock().await;
            config.configuration_number += 1;
            self.storage.lock().await.save_config(&config).await?;
        }

        Ok(accessory)
    }

    async fn remove_accessory(&self, accessory: &pointer::Accessory) -> Result<()> {
        let aid = accessory.lock().await.get_id();

        self.accessory_database
            .lock()
            .await
            .remove_accessory(&accessory)
            .await?;

        let mut aid_cache = self.aid_cache.lock().await;
        if aid_cache.contains(&aid) {
            aid_cache.retain(|id| *id != aid);
            self.storage.lock().await.save_aid_cache(&aid_cache).await?;

            let mut config = self.config.lock().await;
            config.configuration_number += 1;
        }

        Ok(())
    }

    // async fn factory_reset(&mut self) -> Result<()> {
    //     unimplemented!();

    //     Ok(())
    // }
}
