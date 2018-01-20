use std::io::Error;

use db::database::Database;
use db::file_storage::FileStorage;
use pin::Pin;
use protocol::device::Device;

pub struct SecuredDevice {
    device: Device,
    pin: Pin,
}

impl SecuredDevice {
    fn new(name: String, pin: Pin, database: &Database<FileStorage>) -> Result<SecuredDevice, Error> {
        let device = Device::load_or_new(name, database)?;
        Ok(SecuredDevice {
            device: device,
            pin: pin,
        })
    }
}
