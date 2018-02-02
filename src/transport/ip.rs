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

pub struct IpTransport<S: Storage, D: Storage> {
    config: Arc<Config>,
    context: Arc<Mutex<Context>>,
    storage: S,
    database: Database<D>,
    mdns_responder: Responder,
}

impl IpTransport<FileStorage, FileStorage> {
    pub fn new_device(mut config: Config/*, accessory: A*/) -> Result<IpTransport<FileStorage, FileStorage>, Error> {
        let storage = FileStorage::new(&config.storage_path)?;
        let database = Database::new_with_file_storage(&config.storage_path)?;
        config.pin = pin::new(&config.pin)?;

        config.load(&storage);

        let mdns_responder = Responder::new(&config.name, &config.port, config.txt_records());

        let ip_transport = IpTransport {
            config: Arc::new(config),
            context: Arc::new(Mutex::new(Context::new())),
            storage,
            database,
            mdns_responder,
        };

        Ok(ip_transport)
    }
}

impl Transport for IpTransport<FileStorage, FileStorage> {
    fn start(&mut self) -> Result<(), Error> {
        let config = self.config.clone();
        let context = self.context.clone();
        self.mdns_responder.start();
        http::server::serve(SocketAddr::new(self.config.ip, self.config.port), config, context);
        Ok(())
    }

    fn stop(&self) -> Result<(), Error> {
        self.mdns_responder.stop();
        Ok(())
    }
}
