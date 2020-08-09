use hap::{
    accessory::{bridge::BridgeAccessory, lightbulb::LightbulbAccessory, AccessoryCategory, AccessoryInformation},
    characteristic::CharacteristicCallbacks,
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
    let mut lightbulb_1 = LightbulbAccessory::new(2, AccessoryInformation {
        name: "Lightbulb 1".into(),
        ..Default::default()
    })
    .unwrap();
    let mut lightbulb_2 = LightbulbAccessory::new(3, AccessoryInformation {
        name: "Lightbulb 2".into(),
        ..Default::default()
    })
    .unwrap();
    let mut lightbulb_3 = LightbulbAccessory::new(4, AccessoryInformation {
        name: "Lightbulb 3".into(),
        ..Default::default()
    })
    .unwrap();

    lightbulb_1
        .lightbulb
        .on
        .on_update(Some(|current_val: &bool, new_val: &bool| {
            println!(
                "Lightbulb 1: on characteristic updated from {} to {}",
                current_val, new_val
            );
        }));
    lightbulb_2
        .lightbulb
        .on
        .on_update(Some(|current_val: &bool, new_val: &bool| {
            println!(
                "Lightbulb 2: on characteristic updated from {} to {}",
                current_val, new_val
            );
        }));
    lightbulb_3
        .lightbulb
        .on
        .on_update(Some(|current_val: &bool, new_val: &bool| {
            println!(
                "Lightbulb 3: on characteristic updated from {} to {}",
                current_val, new_val
            );
        }));

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

    let mut server = IpServer::new(config, storage).unwrap();
    server.add_accessory(bridge).await.unwrap();
    server.add_accessory(lightbulb_1).await.unwrap();
    server.add_accessory(lightbulb_2).await.unwrap();
    server.add_accessory(lightbulb_3).await.unwrap();

    let handle = server.run_handle();

    std::env::set_var("RUST_LOG", "hap=info");
    env_logger::init();

    handle.await;
}
