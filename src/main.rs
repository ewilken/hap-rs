use std::sync::{Arc, Mutex};

extern crate hap;

use hap::{
    transport::{Transport, ip::IpTransport},
    accessory::{Category, Information, bridge, outlet, door, security_system, valve},
    characteristic::{Readable, Updatable},
    config::Config,
};

pub struct On {
    val: bool,
}

impl Readable<bool> for On {
    fn on_read(&mut self) -> bool {
        println!("On characteristic read.");
        self.val
    }
}

impl Updatable<bool> for On {
    fn on_update(&mut self, old_val: &bool, new_val: &bool) {
        println!("On characteristic set from {} to {}.", old_val, new_val);
    }
}

pub struct DoorPosition {
    val: u8,
}

impl Updatable<u8> for DoorPosition {
    fn on_update(&mut self, old_val: &u8, new_val: &u8) {
        println!("Door position set from {} to {}.", old_val, new_val);
    }
}

fn main() {
    let bridge = bridge::new(Information { name: "Bridge".into(), ..Default::default() });
    let mut outlet = outlet::new(Information { name: "Outlet".into(), ..Default::default() });

    // TODO - fix this
    // let on = Arc::new(Mutex::new(Box::new(On { val: false })));
    // outlet.inner.outlet.inner.on.set_readable(on.clone());
    // outlet.inner.outlet.inner.on.set_updatable(on.clone());
    outlet.inner.outlet.inner.on.set_readable(Some(Arc::new(Mutex::new(Box::new(On { val: false })))));
    outlet.inner.outlet.inner.on.set_updatable(Some(Arc::new(Mutex::new(Box::new(On { val: false })))));

    let mut door = door::new(Information { name: "Door".into(), ..Default::default() });
    door.inner.door.inner.target_position.set_updatable(Some(Arc::new(Mutex::new(Box::new(DoorPosition { val: 0 })))));

    let security_system = security_system::new(Information { name: "Security System".into(), ..Default::default() });
    let valve = valve::new(Information { name: "Valve".into(), ..Default::default() });

    let config = Config {
        name: "Korhal".into(),
        category: Category::Bridge,
        ..Default::default()
    };
    // TODO - take references to the accessories
    let mut ip_transport = IpTransport::new(config, vec![
        Box::new(bridge),
        Box::new(outlet),
        Box::new(door),
        Box::new(security_system),
        Box::new(valve),
    ]).unwrap();

    ip_transport.start().unwrap();
}
