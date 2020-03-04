use std::net::{IpAddr, SocketAddr};

use hap::{
    accessory::{television, Category, Information},
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

    let television = television::new(Information {
        name: "Acme TV".into(),
        ..Default::default()
    })
    .unwrap();

    let config = Config {
        socket_addr: SocketAddr::new(current_ipv4().unwrap(), 32000),
        pin: Pin::from_str("11122333").unwrap(),
        name: "Acme TV".into(),
        category: Category::Television,
        ..Default::default()
    };
    let storage = FileStorage::current_dir().unwrap();

    let mut server = IpServer::new(config, storage).unwrap();
    server.add_accessory(television).unwrap();

    let handle = server.run_handle();

    std::env::set_var("RUST_LOG", "hap=debug");
    env_logger::init();

    handle.await;
}
