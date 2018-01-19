use std::io::Error;
use mdns;

use accessory::AccessoryT;

use config::Config;
use db::storage::Storage;
use db::database::Database;
use db::file_storage::FileStorage;
use pin;

pub struct IpTransport<S: Storage, D: Storage> {
    config: Config,
    storage: S,
    database: Database<D>,
    //responder: mdns::Responder,
}

impl/*<A: AccessoryT>*/ IpTransport<FileStorage, FileStorage> {
    fn new_single_device(mut config: Config/*, accessory: A*/) -> Result<IpTransport<FileStorage, FileStorage>, Error> {
        let storage = FileStorage::new(&config.storage_path)?;
        let database = Database::new_with_file_storage(&config.storage_path)?;
        let pin = pin::new(&config.pin)?;

        config.load(&storage);

        let ip_transport = IpTransport {
            config: config,
            storage: storage,
            database: database,
        };

        Ok(ip_transport)
    }
}
