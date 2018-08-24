use std::{rc::Rc, cell::RefCell, net::SocketAddr};

use config::{Config, ConfigPtr};
use db::{Storage, Database, DatabasePtr, FileStorage, AccessoryList, AccessoryListMember};
use pin;
use protocol::Device;
use transport::{http, mdns::{Responder, ResponderPtr}, bonjour::StatusFlag, Transport};
use event::{Event, Emitter, EmitterPtr};

use Error;

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
    ///     Config,
    ///     accessory::{Category, Information, bridge, lightbulb},
    ///     transport::{Transport, IpTransport},
    /// };
    ///
    /// let config = Config {
    ///     pin: "11122333".into(),
    ///     name: "Acme Lighting".into(),
    ///     category: Category::Bridge,
    /// }
    ///
    /// let bridge_info = Information {
    ///     name: "Bridge".into(),
    ///     ..Default::default()
    /// }
    /// let first_bulb_info = Information {
    ///     name: "Bulb 1".into(),
    ///     ..Default::default()
    /// }
    /// let second_bulb_info = Information {
    ///     name: "Bulb 2".into(),
    ///     ..Default::default()
    /// }
    ///
    /// let bridge = bridge::new(bridge_info).unwrap();
    /// let first_bulb = lightbulb::new(first_bulb_info).unwrap();
    /// let second_bulb = lightbulb::new(second_bulb_info).unwrap();
    ///
    /// let accessories = vec![Box::new(bridge), Box::new(first_bulb), Box::new(second_bulb)];
    ///
    /// let mut ip_transport = IpTransport::new(config, accessories).unwrap();
    ///
    /// ip_transport.start().unwrap();
    /// ```
    pub fn new(
        mut config: Config,
        accessories: Vec<Box<AccessoryListMember>>,
    ) -> Result<IpTransport<FileStorage>, Error> {
        let storage = FileStorage::new(&config.storage_path)?;
        let database = Database::new_with_file_storage(&config.storage_path)?;

        config.load_from(&storage)?;
        config.save_to(&storage)?;

        let pin = pin::new(&config.pin)?;
        let device = Device::load_or_new(config.device_id.to_hex_string(), pin, &database)?;
        let event_emitter = Rc::new(RefCell::new(Emitter::new()));
        let mdns_responder = Rc::new(RefCell::new(Responder::new(&config.name, &config.port, config.txt_records())));

        let mut accessory_list = AccessoryList::new(accessories);
        accessory_list.init_aids(event_emitter.clone())?;

        let ip_transport = IpTransport {
            config: Rc::new(RefCell::new(config)),
            storage,
            database: Rc::new(RefCell::new(database)),
            accessories: accessory_list,
            event_emitter,
            mdns_responder,
        };
        device.save_to(&ip_transport.database)?;

        Ok(ip_transport)
    }
}

impl Transport for IpTransport<FileStorage> {
    fn start(&mut self) -> Result<(), Error> {
        self.mdns_responder.try_borrow_mut()?.start();

        let (ip, port) = {
            let c = self.config.try_borrow()?;
            (c.ip, c.port)
        };

        let config = self.config.clone();
        let database = self.database.clone();
        let mdns_responder = self.mdns_responder.clone();
        self.event_emitter.try_borrow_mut()?.add_listener(Box::new(move |event| {
            match event {
                &Event::DevicePaired => {
                    match database.try_borrow()
                        .expect("couldn't access database")
                        .count_pairings() {
                        Ok(count) => if count > 0 {
                            let mut c = config.try_borrow_mut()
                                .expect("couldn't access config");
                            c.status_flag = StatusFlag::Zero;
                            mdns_responder.try_borrow_mut()
                                .expect("couldn't access mDNS responder")
                                .update_txt_records(c.txt_records())
                                .expect("couldn't update mDNS TXT records");
                        },
                        _ => {},
                    }
                },
                &Event::DeviceUnpaired => {
                    match database.try_borrow()
                        .expect("couldn't access database")
                        .count_pairings() {
                        Ok(count) => if count == 0 {
                            let mut c = config.try_borrow_mut()
                                .expect("couldn't access config");
                            c.status_flag = StatusFlag::NotPaired;
                            mdns_responder.try_borrow_mut()
                                .expect("couldn't access mDNS responder")
                                .update_txt_records(c.txt_records())
                                .expect("couldn't update mDNS TXT records");
                        },
                        _ => {},
                    }
                },
                _ => {},
            }
        }));

        http::server::serve(
            &SocketAddr::new(ip, port),
            self.config.clone(),
            self.database.clone(),
            self.accessories.clone(),
            self.event_emitter.clone(),
        )?;
        Ok(())
    }

    fn stop(&self) -> Result<(), Error> {
        self.mdns_responder.try_borrow()?.stop()?;
        Ok(())
    }
}
