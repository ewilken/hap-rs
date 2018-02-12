use std::io::Error;
use std::sync::{Arc, Mutex};
use transport::mdns::Responder;
use transport::http;
use std::net::SocketAddr;

use accessory::AccessoryT;

use config::Config;
use db::storage::Storage;
use db::database::Database;
use db::file_storage::FileStorage;
use pin;
use db::context::Context;
use protocol::device::Device;
use transport::Transport;

pub struct IpTransport<S: Storage, D: Storage + Send> {
    config: Arc<Config>,
    context: Arc<Mutex<Context>>,
    storage: S,
    database: Arc<Mutex<Database<D>>>,
    mdns_responder: Responder,
}

impl IpTransport<FileStorage, FileStorage> {
    pub fn new_with_device(mut config: Config/*, accessory: A*/) -> Result<IpTransport<FileStorage, FileStorage>, Error> {
        let context = Context::new();
        let storage = FileStorage::new(&config.storage_path)?;
        let database = Database::new_with_file_storage(&config.storage_path)?;

        config.load(&storage);
        config.save(&storage)?;

        let pin = pin::new(&config.pin)?;
        let device = Device::load_or_new(config.device_id.to_hex_string(), pin, &database)?;
        let mdns_responder = Responder::new(&config.name, &config.port, config.txt_records());
        let ip_transport = IpTransport {
            config: Arc::new(config),
            context: Arc::new(Mutex::new(context)),
            storage,
            database: Arc::new(Mutex::new(database)),
            mdns_responder,
        };
        device.save(&ip_transport.database)?;

        Ok(ip_transport)
    }
}

impl Transport for IpTransport<FileStorage, FileStorage> {
    fn start(&mut self) -> Result<(), Error> {
        self.mdns_responder.start();
        http::server::serve::<FileStorage>(
            &SocketAddr::new(self.config.ip, self.config.port),
            self.config.clone(),
            self.database.clone(),
        );
        Ok(())
    }

    fn stop(&self) -> Result<(), Error> {
        self.mdns_responder.stop();
        Ok(())
    }
}
