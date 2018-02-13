use std::net::{IpAddr, Ipv4Addr};
extern crate eui48;
use eui48::MacAddress;

extern crate hap;
use hap::transport::Transport;
use hap::transport::ip::IpTransport;
use hap::accessory::{Information, outlet};
use hap::config::Config;

fn main() {
    let information = Information {
        name: "youcontrol Plug".into(),
        manufacturer: "youcontrol.io".into(),
        serial_number: "12345".into(),
        ..Default::default()
    };
    let outlet = outlet::new(information);

    let config = Config {
        name: "Testoutlet".into(),
        ip: IpAddr::V4(Ipv4Addr::new(192, 168, 42, 69)),
        device_id: MacAddress::parse_str("00:00:12:23:12:67").unwrap(),
        ..Default::default()
    };
    let mut ip_transport = IpTransport::new_with_device(config).unwrap();

    ip_transport.start().unwrap();
}
