use hap::{
    accessory::{bridge::BridgeAccessory, lightbulb::LightbulbAccessory, AccessoryCategory, AccessoryInformation},
    characteristic::CharacteristicCallbacks,
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
    let bridge = BridgeAccessory::new(1, AccessoryInformation {
        name: "Acme Bridge".into(),
        ..Default::default()
    })?;
    let mut lightbulb_1 = LightbulbAccessory::new(2, AccessoryInformation {
        name: "Lightbulb 1".into(),
        ..Default::default()
    })?;
    let mut lightbulb_2 = LightbulbAccessory::new(3, AccessoryInformation {
        name: "Lightbulb 2".into(),
        ..Default::default()
    })?;
    let mut lightbulb_3 = LightbulbAccessory::new(4, AccessoryInformation {
        name: "Lightbulb 3".into(),
        ..Default::default()
    })?;

    lightbulb_1
        .lightbulb
        .power_state
        .on_update(Some(|current_val: &bool, new_val: &bool| {
            println!(
                "Lightbulb 1: power_state characteristic updated from {} to {}",
                current_val, new_val
            );
            Ok(())
        }));
    lightbulb_2
        .lightbulb
        .power_state
        .on_update(Some(|current_val: &bool, new_val: &bool| {
            println!(
                "Lightbulb 2: power_state characteristic updated from {} to {}",
                current_val, new_val
            );
            Ok(())
        }));
    lightbulb_3
        .lightbulb
        .power_state
        .on_update(Some(|current_val: &bool, new_val: &bool| {
            println!(
                "Lightbulb 3: power_state characteristic updated from {} to {}",
                current_val, new_val
            );
            Ok(())
        }));

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
                name: "Acme Bridge".into(),
                device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
                category: AccessoryCategory::Bridge,
                ..Default::default()
            };
            storage.save_config(&config).await?;
            config
        },
    };

    let server = IpServer::new(config, storage).await?;
    server.add_accessory(bridge).await?;
    server.add_accessory(lightbulb_1).await?;
    server.add_accessory(lightbulb_2).await?;
    server.add_accessory(lightbulb_3).await?;

    let handle = server.run_handle();

    std::env::set_var("RUST_LOG", "hap=debug");
    env_logger::init();

    handle.await
}
