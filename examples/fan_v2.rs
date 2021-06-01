use hap::{
    accessory::{fan_v2::FanV2Accessory, AccessoryCategory, AccessoryInformation},
    server::{IpServer, Server},
    storage::{FileStorage, Storage},
    tokio,
    Config,
    MacAddress,
    Pin,
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let fan_v2 = FanV2Accessory::new(1, AccessoryInformation {
        name: "Acme Fan v2".into(),
        ..Default::default()
    })
    .unwrap();

    let mut storage = FileStorage::current_dir().await.unwrap();

    let config = match storage.load_config().await {
        Ok(mut config) => {
            config.redetermine_local_ip();
            storage.save_config(&config).await.unwrap();
            config
        },
        Err(_) => {
            let config = Config {
                pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap(),
                name: "Acme Fan v2".into(),
                device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
                category: AccessoryCategory::Fan,
                ..Default::default()
            };
            storage.save_config(&config).await.unwrap();
            config
        },
    };

    let server = IpServer::new(config, storage).unwrap();
    server.add_accessory(fan_v2).await.unwrap();

    let handle = server.run_handle();

    std::env::set_var("RUST_LOG", "hap=debug");
    env_logger::init();

    handle.await
}
