use std::io::Error;
use std::sync::{Arc, Mutex};
use transport::mdns::Responder;
use transport::http;
use std::net::SocketAddr;

use accessory::{self, HapAccessory};

use config::Config;
use db::storage::Storage;
use db::database::Database;
use db::file_storage::FileStorage;
use pin;
use protocol::device::Device;
use transport::Transport;
use db::accessory_list::{self, AccessoryList, AccessoryListTrait};

pub struct IpTransport<S: Storage, D: Storage + Send> {
    config: Arc<Config>,
    storage: S,
    database: Arc<Mutex<Database<D>>>,
    accessories: AccessoryList,
    mdns_responder: Responder,
}

impl IpTransport<FileStorage, FileStorage> {
    pub fn new(mut config: Config, accessories: Vec<Box<AccessoryListTrait>>) -> Result<IpTransport<FileStorage, FileStorage>, Error> {
        let storage = FileStorage::new(&config.storage_path)?;
        let database = Database::new_with_file_storage(&config.storage_path)?;

        config.load(&storage);
        config.save(&storage)?;

        let pin = pin::new(&config.pin)?;
        let device = Device::load_or_new(config.device_id.to_hex_string(), pin, &database)?;
        let mdns_responder = Responder::new(&config.name, &config.port, config.txt_records());

        let mut a = accessories;
        init_aids(&mut a);

        let ip_transport = IpTransport {
            config: Arc::new(config),
            storage,
            database: Arc::new(Mutex::new(database)),
            accessories: accessory_list::new(a),
            mdns_responder,
        };
        device.save(&ip_transport.database)?;

        Ok(ip_transport)
    }
}

fn init_aids(accessories: &mut Vec<Box<AccessoryListTrait>>) {
    let mut next_aid = 1;
    for accessory in accessories.iter_mut() {
        accessory.set_id(next_aid);
        next_aid += 1;
        accessory::init_iids(accessory);
    }
}

impl Transport for IpTransport<FileStorage, FileStorage> {
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
