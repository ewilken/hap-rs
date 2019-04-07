use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::{
    config::{Config, ConfigPtr},
    db::{AccessoryList, AccessoryListMember, AccessoryListPtr, Database, DatabasePtr, FileStorage, Storage},
    event::{Emitter, EmitterPtr, Event},
    pin,
    protocol::Device,
    transport::{
        bonjour::StatusFlag,
        http,
        mdns::{Responder, ResponderPtr},
        Transport,
    },
};

use crate::Error;

/// Transport via TCP/IP.
pub struct IpTransport<S: Storage> {
    config: ConfigPtr,
    storage: S,
    database: DatabasePtr,
    accessories: AccessoryList,
    event_emitter: EmitterPtr,
    mdns_responder: ResponderPtr,
}

impl IpTransport<FileStorage> {
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
    pub fn new(mut config: Config) -> Result<IpTransport<FileStorage>, Error> {
        let storage = FileStorage::new(&config.storage_path)?;
        let database = Database::new_with_file_storage(&config.storage_path)?;

        config.load_from(&storage)?;
        config.update_hash();
        config.save_to(&storage)?;

        let pin = pin::new(&config.pin)?;
        let device = Device::load_or_new(config.device_id.to_hex_string(), pin, &database)?;
        let event_emitter = Arc::new(Mutex::new(Emitter::new()));
        let mdns_responder = Arc::new(Mutex::new(Responder::new(
            &config.name,
            config.port,
            config.txt_records(),
        )));

        let ip_transport = IpTransport {
            config: Arc::new(Mutex::new(config)),
            storage,
            database: Arc::new(Mutex::new(database)),
            accessories: AccessoryList::new(event_emitter.clone()),
            event_emitter,
            mdns_responder,
        };
        device.save_to(&ip_transport.database)?;

        Ok(ip_transport)
    }
}

impl Transport for IpTransport<FileStorage> {
    fn start(&mut self) -> Result<(), Error> {
        self.mdns_responder
            .lock()
            .expect("couldn't access event_emitter")
            .start();

        let (ip, port) = {
            let c = self.config.lock().expect("couldn't access config");
            (c.ip, c.port)
        };

        let config = self.config.clone();
        let database = self.database.clone();
        let mdns_responder = self.mdns_responder.clone();
        self.event_emitter
            .lock()
            .expect("couldn't access event_emitter")
            .add_listener(Box::new(move |event| match *event {
                Event::DevicePaired => {
                    if let Ok(count) = database.lock().expect("couldn't access database").count_pairings() {
                        if count > 0 {
                            let mut c = config.lock().expect("couldn't access config");
                            c.status_flag = StatusFlag::Zero;
                            mdns_responder
                                .lock()
                                .expect("couldn't access mDNS responder")
                                .update_txt_records(c.txt_records())
                                .expect("couldn't update mDNS TXT records");
                        }
                    }
                },
                Event::DeviceUnpaired => {
                    if let Ok(count) = database.lock().expect("couldn't access database").count_pairings() {
                        if count == 0 {
                            let mut c = config.lock().expect("couldn't access config");
                            c.status_flag = StatusFlag::NotPaired;
                            mdns_responder
                                .lock()
                                .expect("couldn't access mDNS responder")
                                .update_txt_records(c.txt_records())
                                .expect("couldn't update mDNS TXT records");
                        }
                    }
                },
                _ => {},
            }));

        http::server::serve(
            &SocketAddr::new(ip, port),
            &self.config,
            &self.database,
            &self.accessories,
            &self.event_emitter,
        )?;
        Ok(())
    }

    fn stop(&self) -> Result<(), Error> {
        self.mdns_responder
            .lock()
            .expect("couldn't access mDNS responder")
            .stop()?;
        Ok(())
    }

    fn add_accessory<A: 'static + AccessoryListMember + Send>(
        &mut self,
        accessory: A,
    ) -> Result<AccessoryListPtr, Error> {
        self.accessories.add_accessory(Box::new(accessory))
    }

    fn remove_accessory(&mut self, accessory: &AccessoryListPtr) -> Result<(), Error> {
        self.accessories.remove_accessory(accessory)
    }
}
