use std::sync::{Arc, Mutex};

extern crate hap;

use hap::{
    transport::{Transport, ip::IpTransport},
    accessory::{Information, outlet},
    characteristic::{Readable, Updatable},
    config::Config,
};

pub struct On {
    val: bool,
}

impl Readable<bool> for On {
    fn on_read(&mut self) -> Option<bool> {
        self.val = !self.val;
        println!("On characteristic read and turned to {}.", &self.val);
        Some(self.val)
    }
}

impl Updatable<bool> for On {
    fn on_update(&mut self, old_val: &Option<bool>, new_val: &bool) {
        if let &Some(old_val) = old_val {
            println!("On characteristic set from {} to {}.", old_val, new_val);
        } else {
            println!("On characteristic set to {}.", new_val);
        }
    }
}

fn main() {
    let information = Information {
        name: "Test".into(),
        manufacturer: "Korhal".into(),
        serial_number: "12345".into(),
        ..Default::default()
    };
    let mut outlet = outlet::new(information);

    // TODO - fix this
    // let on = Arc::new(Mutex::new(Box::new(On { val: false })));
    // outlet.inner.outlet.inner.on.set_readable(on.clone());
    // outlet.inner.outlet.inner.on.set_updatable(on.clone());
    outlet.inner.outlet.inner.on.set_readable(Arc::new(Mutex::new(Box::new(On { val: false }))));
    outlet.inner.outlet.inner.on.set_updatable(Arc::new(Mutex::new(Box::new(On { val: false }))));

    let config = Config {
        name: "Test".into(),
        ..Default::default()
    };
    // TODO - take a reference to the outlet
    let mut ip_transport = IpTransport::new(config, vec![Box::new(outlet)]).unwrap();

    ip_transport.start().unwrap();
}
