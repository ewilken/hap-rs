use std::net::{IpAddr, SocketAddr};

use hap::{
    accessory::{lightbulb::LightbulbAccessory, AccessoryCategory, AccessoryInformation},
    characteristic::AsyncCharacteristicCallbacks,
    futures::future::FutureExt,
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

    let mut lightbulb = LightbulbAccessory::new(1, AccessoryInformation {
        name: "Acme Lightbulb".into(),
        ..Default::default()
    })
    .unwrap();

    lightbulb.lightbulb.on.on_read_async(Some(|| {
        async {
            println!("on characteristic read");
            None
        }
        .boxed()
    }));
    lightbulb
        .lightbulb
        .on
        .on_update_async(Some(|current_val: bool, new_val: bool| {
            async move {
                println!("on characteristic updated from {} to {}", current_val, new_val);
            }
            .boxed()
        }));

    let config = Config {
        socket_addr: SocketAddr::new(current_ipv4().unwrap(), 32000),
        pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap(),
        name: "Acme Lightbulb".into(),
        device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
        category: AccessoryCategory::Lightbulb,
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
