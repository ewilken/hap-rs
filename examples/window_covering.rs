use hap::{
    accessory::{window_covering::WindowCoveringAccessory, AccessoryCategory, AccessoryInformation},
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
    let window_covering = WindowCoveringAccessory::new(1, AccessoryInformation {
        name: "Acme Window Covering".into(),
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
                name: "Acme Window Covering".into(),
                device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
                category: AccessoryCategory::WindowCovering,
                ..Default::default()
            };
            storage.save_config(&config).await.unwrap();
            config
        },
    };

    let server = IpServer::new(config, storage).unwrap();
    server.add_accessory(window_covering).await.unwrap();

    let handle = server.run_handle();

    std::env::set_var("RUST_LOG", "hap=debug");
    env_logger::init();

    handle.await
}
