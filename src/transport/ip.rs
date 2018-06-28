use std::{io::Error, rc::Rc, cell::RefCell, net::SocketAddr};

use config::{Config, ConfigPtr};
use db::{
    storage::Storage,
    database::{Database, DatabasePtr},
    file_storage::FileStorage,
    accessory_list::{AccessoryList, AccessoryListTrait},
};
use pin;
use protocol::device::Device;
use transport::{http, mdns::Responder, bonjour::StatusFlag, Transport};
use event::{Event, Emitter, EmitterPtr};

pub struct IpTransport<S: Storage> {
    config: ConfigPtr,
    storage: S,
    database: DatabasePtr,
    accessories: AccessoryList,
    event_emitter: EmitterPtr,
    mdns_responder: Responder,
}

impl IpTransport<FileStorage> {
    pub fn new(
        mut config: Config,
        accessories: Vec<Box<AccessoryListTrait>>,
    ) -> Result<IpTransport<FileStorage>, Error> {
        let storage = FileStorage::new(&config.storage_path)?;
        let database = Database::new_with_file_storage(&config.storage_path)?;

        config.load(&storage);
        config.save(&storage)?;

        let pin = pin::new(&config.pin)?;
        let device = Device::load_or_new(config.device_id.to_hex_string(), pin, &database)?;
        let event_emitter = Rc::new(RefCell::new(Emitter::new()));
        let mdns_responder = Responder::new(&config.name, &config.port, config.txt_records());

        let mut accessory_list = AccessoryList::new(accessories);
        accessory_list.init_aids(event_emitter.clone());

        let ip_transport = IpTransport {
            config: Rc::new(RefCell::new(config)),
            storage,
            database: Rc::new(RefCell::new(database)),
            accessories: accessory_list,
            event_emitter,
            mdns_responder,
        };
        device.save(&ip_transport.database)?;

        Ok(ip_transport)
    }
}

impl Transport for IpTransport<FileStorage> {
    fn start(&mut self) -> Result<(), Error> {
        self.mdns_responder.start();

        let (ip, port) = {
            let c = self.config.borrow();
            (c.ip, c.port)
        };

        let config = self.config.clone();
        let database = self.database.clone();
        self.event_emitter.borrow_mut().add_listener(Box::new(move |event| {
            match event {
                &Event::DevicePaired => {
                    match database.borrow().count_pairings() {
                        Ok(count) => if count > 0 {
                            config.borrow_mut().status_flag = StatusFlag::Zero;
                            // TODO - update MDNS txt records
                        },
                        _ => {},
                    }
                },
                &Event::DeviceUnpaired => {
                    match database.borrow().count_pairings() {
                        Ok(count) => if count == 0 {
                            config.borrow_mut().status_flag = StatusFlag::NotPaired;
                            // TODO - update MDNS txt records
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
        );
        Ok(())
    }

    fn stop(&self) -> Result<(), Error> {
        self.mdns_responder.stop();
        Ok(())
    }
}
