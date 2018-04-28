use std::sync::{Arc, Mutex};

pub enum Event {
    DevicePaired,
    DeviceUnpaired,
    CharacteristicValueChanged { aid: u64, iid: u64 }
}

pub struct Emitter<'e> {
    listeners: Vec<Arc<Mutex<Box<&'e mut Listener>>>>,
}

impl<'e> Emitter<'e> {
    pub fn new() -> Emitter<'e> {
        Emitter { listeners: vec![] }
    }

    pub fn add_listener(&mut self, listener: Arc<Mutex<Box<&'e mut Listener>>>) {
        self.listeners.push(listener);
    }

    pub fn emit(&mut self, event: Event) {
        for listener in self.listeners.iter_mut() {
            listener.lock().unwrap().handle(&event);
        }
    }
}

pub trait Listener {
    fn handle(&mut self, event: &Event);
}
