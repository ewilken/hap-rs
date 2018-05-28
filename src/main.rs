use std::sync::{Arc, Mutex};

extern crate hap;

use hap::{
    transport::{Transport, ip::IpTransport},
    accessory::{Information, bridge, outlet, garage_door_opener},
    characteristic::{Readable, Updatable},
    config::Config,
};

pub struct On {
    val: bool,
}

impl Readable<bool> for On {
    fn on_read(&mut self) -> bool {
        self.val = !self.val;
        println!("On characteristic read and turned to {}.", &self.val);
        self.val
    }
}

impl Updatable<bool> for On {
    fn on_update(&mut self, old_val: &bool, new_val: &bool) {
        println!("On characteristic set from {} to {}.", old_val, new_val);
    }
}

fn main() {
    let bridge_i = Information {
        name: "Korhal Bridge".into(),
        manufacturer: "Korhal".into(),
        serial_number: "12345".into(),
        ..Default::default()
    };
    let bridge = bridge::new(bridge_i);

    let outlet_i = Information {
        name: "Korhal Outlet".into(),
        manufacturer: "Korhal".into(),
        serial_number: "23456".into(),
        ..Default::default()
    };
    let mut outlet = outlet::new(outlet_i);

    let garage_door_opener_i = Information {
        name: "Korhal Garage".into(),
        manufacturer: "Korhal".into(),
        serial_number: "34567".into(),
        ..Default::default()
    };
    let garage_door_opener = garage_door_opener::new(garage_door_opener_i);

    // TODO - fix this
    // let on = Arc::new(Mutex::new(Box::new(On { val: false })));
    // outlet.inner.outlet.inner.on.set_readable(on.clone());
    // outlet.inner.outlet.inner.on.set_updatable(on.clone());
    outlet.inner.outlet.inner.on.set_readable(Some(Arc::new(Mutex::new(Box::new(On { val: false })))));
    outlet.inner.outlet.inner.on.set_updatable(Some(Arc::new(Mutex::new(Box::new(On { val: false })))));

    let config = Config {
        name: "Korhal".into(),
        ..Default::default()
    };
    // TODO - take references to the accessories
    let mut ip_transport = IpTransport::new(config, vec![
        Box::new(bridge),
        Box::new(outlet),
        Box::new(garage_door_opener)
    ]).unwrap();

    ip_transport.start().unwrap();
}
