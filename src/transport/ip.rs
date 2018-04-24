use std::{io::Error, sync::{Arc, Mutex}, net::SocketAddr};

use config::{Config, ConfigPtr};
use db::{
    storage::Storage,
    database::{Database, DatabasePtr},
    file_storage::FileStorage,
    accessory_list::{AccessoryList, AccessoryListTrait},
};
use pin;
use protocol::device::Device;
use transport::{http, mdns::Responder, Transport};

pub struct IpTransport<S: Storage> {
    config: ConfigPtr,
    storage: S,
    database: DatabasePtr,
    accessories: AccessoryList,
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
        let mdns_responder = Responder::new(&config.name, &config.port, config.txt_records());

        let mut accessory_list = AccessoryList::new(accessories);
        accessory_list.init_aids();

        let ip_transport = IpTransport {
            config: Arc::new(config),
            storage,
            database: Arc::new(Mutex::new(database)),
            accessories: accessory_list,
            mdns_responder,
        };
        device.save(&ip_transport.database)?;

        Ok(ip_transport)
    }
}

impl Transport for IpTransport<FileStorage> {
    fn start(&mut self) -> Result<(), Error> {
        self.mdns_responder.start();
        http::server::serve::<FileStorage>(
            &SocketAddr::new(self.config.ip, self.config.port),
            self.config.clone(),
            self.database.clone(),
            self.accessories.clone(),
        );
        Ok(())
    }

    fn stop(&self) -> Result<(), Error> {
        self.mdns_responder.stop();
        Ok(())
    }
}
