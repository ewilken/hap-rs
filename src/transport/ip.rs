use std::io::Error;
use transport::mdns::Responder;
use transport::http;
use std::net::SocketAddr;

use accessory::AccessoryT;

use config::Config;
use db::storage::Storage;
use db::database::Database;
use db::file_storage::FileStorage;
use pin;
use protocol::context::Context;
use protocol::device::Device;
use protocol::secured_device::SecuredDevice;
use transport::Transport;

pub struct IpTransport<S: Storage, D: Storage> {
    config: Config,
    //context: Context,
    storage: S,
    database: Database<D>,
    secured_device: SecuredDevice,
    mdns_responder: Responder,
}

impl IpTransport<FileStorage, FileStorage> {
    pub fn new_single_device(mut config: Config/*, accessory: A*/) -> Result<IpTransport<FileStorage, FileStorage>, Error> {
        let storage = FileStorage::new(&config.storage_path)?;
        let database = Database::new_with_file_storage(&config.storage_path)?;
        let pin = pin::new(&config.pin)?;

        config.load(&storage);

        let secured_device = SecuredDevice::new(&config.name, pin, &database)?;
        let mdns_responder = Responder::new(&config.name, config.txt_records());

        let ip_transport = IpTransport {
            config,
            //context: Context::new(),
            storage,
            database,
            secured_device,
            mdns_responder,
        };

        Ok(ip_transport)
    }
}

impl Transport for IpTransport<FileStorage, FileStorage> {
    fn start(&mut self) -> Result<(), Error> {
        self.mdns_responder.start();
        http::server::serve(SocketAddr::new(self.config.ip, self.config.port));
        Ok(())
    }

    fn stop(&self) -> Result<(), Error> {
        self.mdns_responder.stop();
        Ok(())
    }
}
