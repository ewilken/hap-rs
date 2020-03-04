use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::{event, storage};

pub type ControllerId = Arc<Mutex<Option<Uuid>>>;

pub type EventEmitter = Arc<Mutex<event::EventEmitter>>;

pub type EventSubscriptions = Arc<Mutex<Vec<(u64, u64)>>>;

pub type AccessoryList = Arc<Mutex<storage::accessory_list::AccessoryList>>;

pub type AccessoryListMember = Arc<Mutex<Box<dyn storage::accessory_list::AccessoryListMember + Send>>>;

pub type Storage = Arc<Mutex<Box<dyn storage::Storage + Send>>>;

pub type Config = Arc<Mutex<crate::Config>>;
