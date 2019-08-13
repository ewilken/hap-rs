use std::sync::{Arc, Mutex};

use serde_json::Value;

pub enum Event {
    DevicePaired,
    DeviceUnpaired,
    CharacteristicValueChanged { aid: u64, iid: u64, value: Value },
}

#[derive(Default)]
pub struct EventEmitter {
    listeners: Vec<Box<dyn Fn(&Event) + Send>>,
}

impl EventEmitter {
    pub fn new() -> EventEmitter { EventEmitter { listeners: vec![] } }

    pub fn add_listener(&mut self, listener: Box<dyn Fn(&Event) + Send>) { self.listeners.push(listener); }

    pub fn emit(&self, event: &Event) {
        for listener in &self.listeners {
            listener(&event);
        }
    }
}

/// Pointer to an `EventEmitter`.
pub type EventEmitterPtr = Arc<Mutex<EventEmitter>>;

/// Pointer to a list of event subscriptions.
pub type EventSubscriptions = Arc<Mutex<Vec<(u64, u64)>>>;
