use std::net::{IpAddr, SocketAddr};

use hap::{
    accessory::{lightbulb::LightbulbAccessory, Category, Information},
    characteristic::CharacteristicCallbacks,
    server::{IpServer, Server},
    storage::FileStorage,
    tokio,
    Config,
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

    let mut lightbulb = LightbulbAccessory::new(1, Information {
        name: "Lightbulb".into(),
        ..Default::default()
    })
    .unwrap();

    lightbulb.lightbulb.on.on_read(|| {
        println!("on characteristic read");
        None
    });
    lightbulb.lightbulb.on.on_update(|old_val: &bool, new_val: &bool| {
        println!("on characteristic updated from {} to {}", old_val, new_val);
    });

    let config = Config {
        socket_addr: SocketAddr::new(current_ipv4().unwrap(), 32000),
        pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap(),
        name: "Acme Lightbulb".into(),
        device_id: eui48::MacAddress::new([1, 2, 3, 4, 5, 6]),
        category: Category::Lightbulb,
        ..Default::default()
    };
    let storage = FileStorage::current_dir().await.unwrap();

    let mut server = IpServer::new(config, storage).unwrap();
    server.add_accessory(lightbulb).await.unwrap();

    let handle = server.run_handle();

    std::env::set_var("RUST_LOG", "hap=debug");
    env_logger::init();

    handle.await;
}
