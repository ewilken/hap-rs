use std::sync::{Arc, Mutex};

use futures::future::{self, BoxFuture, FutureExt};

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
    pub fn new<S: Storage + Send + 'static>(config: Config, storage: S) -> Result<Self> {
        let config = Arc::new(Mutex::new(config));
        let storage: pointer::Storage = Arc::new(Mutex::new(Box::new(storage)));

        let config_ = config.clone();
        let storage_ = storage.clone();
        let mut event_emitter = EventEmitter::new();
        event_emitter.add_listener(Box::new(move |event| match *event {
            Event::DevicePaired => {
                if let Ok(count) = storage_.lock().expect("couldn't access storage").count_pairings() {
                    if count > 0 {
                        let mut c = config_.lock().expect("couldn't access config");
                        c.status_flag = BonjourStatusFlag::Zero;
                    }
                }
            },
            Event::DeviceUnpaired => {
                if let Ok(count) = storage_.lock().expect("couldn't access storage").count_pairings() {
                    if count == 0 {
                        let mut c = config_.lock().expect("couldn't access config");
                        c.status_flag = BonjourStatusFlag::NotPaired;
                    }
                }
            },
            _ => {},
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

impl Server for IpServer {
    fn run_handle(&self) -> BoxFuture<()> {
        let http_handle = self.http_server.run_handle();
        let mdns_handle = self.mdns_responder.run_handle();

        Box::pin(future::join(http_handle, mdns_handle).map(|_| ()).boxed())
    }

    fn config_pointer(&self) -> pointer::Config { self.config.clone() }

    fn storage_pointer(&self) -> pointer::Storage { self.storage.clone() }

    fn add_accessory<A: 'static + AccessoryListMember + Send>(
        &mut self,
        accessory: A,
    ) -> Result<pointer::AccessoryListMember> {
        let accessory = self
            .accessory_list
            .lock()
            .expect("couldn't access accessory list")
            .add_accessory(Box::new(accessory))?;

        let mut config = self.config.lock().expect("couldn't access config");
        config.configuration_number += 1;

        Ok(accessory)
    }

    fn remove_accessory(&mut self, accessory: &pointer::AccessoryListMember) -> Result<()> {
        self.accessory_list
            .lock()
            .expect("couldn't access accessory list")
            .remove_accessory(accessory)?;

        let mut config = self.config.lock().expect("couldn't access config");
        config.configuration_number += 1;

        Ok(())
    }
}
