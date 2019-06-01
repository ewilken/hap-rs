use hap::{
    accessory::{television, Category, Information},
    transport::{IpTransport, Transport},
    Config,
};

fn main() {
    let television = television::new(Information {
        name: "Acme TV".into(),
        ..Default::default()
    })
    .unwrap();

    let mut ip_transport = IpTransport::new(Config {
        name: "Acme TV".into(),
        category: Category::Television,
        ..Default::default()
    })
    .unwrap();
    ip_transport.add_accessory(television).unwrap();

    ip_transport.start().unwrap();
}
