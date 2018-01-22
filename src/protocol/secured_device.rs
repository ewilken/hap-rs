use std::io::Error;
use uuid::Uuid;

use db::database::Database;
use db::file_storage::FileStorage;
use pin::Pin;
use protocol::device::Device;

pub struct SecuredDevice {
    device: Device,
    pin: Pin,
}

impl SecuredDevice {
    pub fn new(id: Uuid, pin: Pin, database: &Database<FileStorage>) -> Result<SecuredDevice, Error> {
        let device = Device::load_or_new(id, database)?;
        Ok(SecuredDevice {
            device: device,
            pin: pin,
        })
    }
}
