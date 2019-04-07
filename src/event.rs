use std::sync::{Arc, Mutex};

use serde_json::Value;

pub enum Event {
    DevicePaired,
    DeviceUnpaired,
    CharacteristicValueChanged { aid: u64, iid: u64, value: Value },
}

#[derive(Default)]
pub struct Emitter {
    listeners: Vec<Box<dyn Fn(&Event) + Send>>,
}

impl Emitter {
    pub fn new() -> Emitter { Emitter { listeners: vec![] } }

    pub fn add_listener(&mut self, listener: Box<dyn Fn(&Event) + Send>) { self.listeners.push(listener); }

    pub fn emit(&self, event: &Event) {
        for listener in &self.listeners {
            listener(&event);
        }
    }
}

/// Reference counting pointer to an `Emitter`.
pub type EmitterPtr = Arc<Mutex<Emitter>>;
