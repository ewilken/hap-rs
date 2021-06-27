use futures::lock::Mutex;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use crate::{accessory, event, storage};

pub type ControllerId = Arc<RwLock<Option<Uuid>>>;

pub type EventEmitter = Arc<Mutex<event::EventEmitter>>;

pub type EventSubscriptions = Arc<Mutex<Vec<(u64, u64)>>>;

pub type AccessoryDatabase = Arc<Mutex<storage::accessory_database::AccessoryDatabase>>;

pub type Accessory = Arc<Mutex<Box<dyn accessory::HapAccessory>>>;

pub type Storage = Arc<Mutex<Box<dyn storage::Storage>>>;

pub type Config = Arc<Mutex<crate::Config>>;

pub type MdnsResponder = Arc<Mutex<crate::transport::mdns::MdnsResponder>>;
