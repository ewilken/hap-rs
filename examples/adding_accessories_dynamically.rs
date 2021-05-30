use hap::{
    accessory::{bridge::BridgeAccessory, lightbulb::LightbulbAccessory, AccessoryCategory, AccessoryInformation},
    server::{IpServer, Server},
    storage::{FileStorage, Storage},
    tokio,
    Config,
    MacAddress,
    Pin,
};

#[tokio::main]
async fn main() {
    let bridge = BridgeAccessory::new(1, AccessoryInformation {
        name: "Acme Bridge".into(),
        ..Default::default()
    })
    .unwrap();
    let lightbulb = LightbulbAccessory::new(2, AccessoryInformation {
        name: "Acme Lightbulb".into(),
        ..Default::default()
    })
    .unwrap();

    let mut storage = FileStorage::current_dir().await.unwrap();

    let config = match storage.load_config().await {
        Ok(config) => config,
        Err(_) => {
            let config = Config {
                pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap(),
                name: "Acme Bridge".into(),
                device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
                category: AccessoryCategory::Bridge,
                ..Default::default()
            };
            storage.save_config(&config).await.unwrap();
            config
        },
    };

    let server = IpServer::new(config, storage).unwrap();
    server.add_accessory(bridge).await.unwrap();
    server.add_accessory(lightbulb).await.unwrap();

    let handle = server.run_handle();

    let stream_of_new_accessories = async {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;

        for i in 0..20 {
            let lightbulb = LightbulbAccessory::new(i + 3, AccessoryInformation {
                name: format!("Another Lightbulb {}", i + 1),
                ..Default::default()
            })
            .unwrap();

            server.add_accessory(lightbulb).await.unwrap();

            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        }
    };

    std::env::set_var("RUST_LOG", "hap=debug");
    env_logger::init();

    futures::join!(handle, stream_of_new_accessories);
}
