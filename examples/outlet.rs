use hap::{
    accessory::{outlet::OutletAccessory, AccessoryCategory, AccessoryInformation},
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
    let outlet = OutletAccessory::new(1, AccessoryInformation {
        name: "Acme Outlet".into(),
        ..Default::default()
    })?;

    let mut storage = FileStorage::current_dir().await?;

    let config = match storage.load_config().await {
        Ok(mut config) => {
            config.redetermine_local_ip();
            storage.save_config(&config).await?;
            config
        },
        Err(_) => {
            let config = Config {
                pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3])?,
                name: "Acme Outlet".into(),
                device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
                category: AccessoryCategory::Outlet,
                ..Default::default()
            };
            storage.save_config(&config).await?;
            config
        },
    };

    let server = IpServer::new(config, storage).await?;
    server.add_accessory(outlet).await?;

    let handle = server.run_handle();

    std::env::set_var("RUST_LOG", "hap=debug");
    env_logger::init();

    handle.await
}
