extern crate hap;

use hap::{
    transport::{Transport, ip::IpTransport},
    accessory::{Information, outlet},
    config::Config
};

fn turn_on() -> Option<bool> {
    println!("On characteristic read and turned to true.");
    Some(true)
}

fn print_val(old: &Option<bool>, new: &bool) {
    if let &Some(old) = old {
        println!("On characteristic set from {} to {}.", old, new);
    } else {
        println!("On characteristic set to {}.", new);
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

    outlet.inner.outlet.inner.on.on_read(Box::new(|| turn_on()));
    outlet.inner.outlet.inner.on.on_update(Box::new(|old, new| print_val(old, new)));

    let config = Config {
        name: "Test".into(),
        ..Default::default()
    };
    let mut ip_transport = IpTransport::new(config, vec![Box::new(outlet)]).unwrap();

    ip_transport.start().unwrap();
}
