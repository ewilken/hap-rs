use std::{rc::Rc, cell::RefCell};

use serde_json::Value;

pub enum Event {
    DevicePaired,
    DeviceUnpaired,
    CharacteristicValueChanged { aid: u64, iid: u64, value: Value }
}

pub struct Emitter {
    listeners: Vec<Box<Fn(&Event)>>,
}

impl Emitter {
    pub fn new() -> Emitter {
        Emitter { listeners: vec![] }
    }

    pub fn add_listener(&mut self, listener: Box<Fn(&Event)>) {
        self.listeners.push(listener);
    }

    pub fn emit(&self, event: Event) {
        for listener in self.listeners.iter() {
            listener(&event);
        }
    }
}

/// Reference counting pointer to an `Emitter`.
pub type EmitterPtr = Rc<RefCell<Emitter>>;
