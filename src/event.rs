use std::fmt::Debug;

use futures::future::BoxFuture;
use serde_json::Value;

#[derive(Debug)]
pub enum Event {
    DevicePaired,
    DeviceUnpaired,
    CharacteristicValueChanged { aid: u64, iid: u64, value: Value },
}

#[derive(Default)]
pub struct EventEmitter {
    listeners: Vec<Box<dyn (Fn(&Event) -> BoxFuture<()>) + Send + Sync>>,
}

impl EventEmitter {
    pub fn new() -> EventEmitter { EventEmitter { listeners: vec![] } }

    pub fn add_listener(&mut self, listener: Box<dyn (Fn(&Event) -> BoxFuture<()>) + Send + Sync>) {
        self.listeners.push(listener);
    }

    pub async fn emit(&self, event: &Event) {
        for listener in &self.listeners {
            listener(&event).await;
        }
    }
}
