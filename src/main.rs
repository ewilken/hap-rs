extern crate hap;

fn main() {
    let information = hap::accessory::Information {
        name: "youcontrol outlet".into(),
        manufacturer: "youcontrol.io".into(),
        serial_number: "12345".into(),
        ..Default::default()
    };
    let outlet = hap::accessory::outlet::new(information);

    let config = hap::config::Config {
        ..Default::default()
    };
    let ip_transport = hap::transport::ip::IpTransport::new_with_file_storage(config);

    ip_transport.start().unwrap();
}
