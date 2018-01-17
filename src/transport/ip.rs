use std::io::Error;
use mdns;

use accessory::AccessoryT;

use config::Config;
use db::storage::Storage;
use db::database::Database;
use db::file_storage::FileStorage;

pub struct IpTransport<S: Storage, D: Storage> {
    config: Config,
    storage: S,
    database: Database<D>,
    //responder: mdns::Responder,
}

// TODO - this is shit.
impl/*<A: AccessoryT>*/ IpTransport<FileStorage, FileStorage> {
    fn new_single_device(config: Config/*, accessory: A*/) -> Result<IpTransport<FileStorage, FileStorage>, Error> {
        let storage = FileStorage::new(&config.storage_path)?;
        let database = Database::new_with_file_storage(&config.storage_path)?;

        Ok(IpTransport {
            config: config,
            storage: storage,
            database: database,
        })
    }
}
