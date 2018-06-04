use std::sync::{Arc, Mutex};

extern crate hap;

use hap::{
    transport::{Transport, ip::IpTransport},
    accessory::{Category, Information, outlet},
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
    let mut outlet = outlet::new(Information { name: "Korhal Outlet".into(), ..Default::default() });

    // TODO - fix this
    // let on = Arc::new(Mutex::new(Box::new(On { val: false })));
    // outlet.inner.outlet.inner.on.set_readable(on.clone());
    // outlet.inner.outlet.inner.on.set_updatable(on.clone());
    outlet.inner.outlet.inner.on.set_readable(Some(Arc::new(Mutex::new(Box::new(On { val: false })))));
    outlet.inner.outlet.inner.on.set_updatable(Some(Arc::new(Mutex::new(Box::new(On { val: false })))));

    let config = Config {
        name: "Korhal Outlet".into(),
        category: Category::Outlet,
        ..Default::default()
    };
    // TODO - take references to the accessories
    let mut ip_transport = IpTransport::new(config, vec![Box::new(outlet)]).unwrap();

    ip_transport.start().unwrap();
}
