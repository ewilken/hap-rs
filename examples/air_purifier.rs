use std::net::{IpAddr, SocketAddr};

use hap::{
    accessory::{air_purifier::AirPurifierAccessory, AccessoryCategory, AccessoryInformation},
    server::{IpServer, Server},
    storage::FileStorage,
    tokio,
    Config,
    MacAddress,
    Pin,
};

#[tokio::main]
async fn main() {
    let current_ipv4 = || -> Option<IpAddr> {
        for iface in pnet::datalink::interfaces() {
            for ip_network in iface.ips {
                if ip_network.is_ipv4() {
                    let ip = ip_network.ip();
                    if !ip.is_loopback() {
                        return Some(ip);
                    }
                }
            }
        }
        None
    };

    let lightbulb = AirPurifierAccessory::new(1, AccessoryInformation {
        name: "Acme Air Purifier".into(),
        ..Default::default()
    })
    .unwrap();

    let config = Config {
        socket_addr: SocketAddr::new(current_ipv4().unwrap(), 32000),
        pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap(),
        name: "Acme Air Purifier".into(),
        device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
        category: AccessoryCategory::AirPurifier,
        ..Default::default()
    };
    let storage = FileStorage::current_dir().await.unwrap();

    let mut server = IpServer::new(config, storage).unwrap();
    server.add_accessory(lightbulb).await.unwrap();

    let handle = server.run_handle();

    std::env::set_var("RUST_LOG", "hap=info");
    env_logger::init();

    handle.await;
}
