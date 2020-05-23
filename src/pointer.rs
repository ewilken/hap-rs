use std::sync::{Arc, RwLock};

use futures::lock::Mutex;
use uuid::Uuid;

use crate::{event, storage};

pub type ControllerId = Arc<RwLock<Option<Uuid>>>;

pub type EventEmitter = Arc<Mutex<event::EventEmitter>>;

pub type EventSubscriptions = Arc<Mutex<Vec<(u64, u64)>>>;

pub type AccessoryList = Arc<Mutex<storage::accessory_list::AccessoryList>>;

pub type AccessoryListMember = Arc<Mutex<Box<dyn storage::accessory_list::AccessoryListMember + Send + Sync>>>;

pub type Storage = Arc<Mutex<Box<dyn storage::Storage + Send + Sync>>>;

pub type Config = Arc<Mutex<crate::Config>>;
