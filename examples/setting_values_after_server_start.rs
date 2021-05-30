use hap::{
    accessory::{motion_sensor::MotionSensorAccessory, AccessoryCategory, AccessoryInformation},
    serde_json::Value,
    server::{IpServer, Server},
    storage::{FileStorage, Storage},
    tokio,
    Config,
    HapType,
    MacAddress,
    Pin,
};

#[tokio::main]
async fn main() {
    let sensor = MotionSensorAccessory::new(1, AccessoryInformation {
        name: "Acme Sensor".into(),
        ..Default::default()
    })
    .unwrap();

    let mut storage = FileStorage::current_dir().await.unwrap();

    let config = match storage.load_config().await {
        Ok(config) => config,
        Err(_) => {
            let config = Config {
                pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap(),
                name: "Acme Sensor".into(),
                device_id: MacAddress::new([10, 20, 30, 40, 50, 63]),
                category: AccessoryCategory::Sensor,
                ..Default::default()
            };
            storage.save_config(&config).await.unwrap();
            config
        },
    };

    let server = IpServer::new(config, storage).unwrap();
    let sensor_ptr = server.add_accessory(sensor).await.unwrap();

    let handle = server.run_handle();

    let value_set_interval = async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(2));

        tokio::time::sleep(std::time::Duration::from_secs(60)).await;

        loop {
            interval.tick().await;

            let mut motion_sensor_accessory = sensor_ptr.lock().await;
            let motion_sensor_service = motion_sensor_accessory.get_mut_service(HapType::MotionSensor).unwrap();
            let motion_detected_characteristic = motion_sensor_service
                .get_mut_characteristic(HapType::MotionDetected)
                .unwrap();

            motion_detected_characteristic
                .set_value(Value::Bool(true))
                .await
                .unwrap();

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            motion_detected_characteristic
                .set_value(Value::Bool(false))
                .await
                .unwrap();
        }
    };

    std::env::set_var("RUST_LOG", "hap=debug");
    env_logger::init();

    futures::join!(handle, value_set_interval);
}
