extern crate hap;
use hap::transport::Transport;
use hap::transport::ip::IpTransport;
use hap::accessory::{Information, outlet};
use hap::characteristic::Updatable;
use hap::config::Config;

struct VirtualOutlet {}

impl Updatable<bool> for VirtualOutlet {
    fn on_update(&mut self, val: &bool) {
        println!("On characteristic set to {}.", val);
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

    let virtual_outlet = VirtualOutlet {};
    outlet.inner.outlet.inner.on.set_updatable(Box::new(virtual_outlet));

    let config = Config {
        name: "Test".into(),
        ..Default::default()
    };
    let mut ip_transport = IpTransport::new(config, vec![Box::new(outlet)]).unwrap();

    ip_transport.start().unwrap();
}
