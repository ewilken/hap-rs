use tokio;

use hap::{
    accessory::{motion_sensor::MotionSensorAccessory, AccessoryCategory, AccessoryInformation},
    serde_json::Value,
    server::{IpServer, Server},
    storage::{FileStorage, Storage},
    Config,
    HapType,
    MacAddress,
    Pin,
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let sensor = MotionSensorAccessory::new(1, AccessoryInformation {
        name: "Acme Sensor".into(),
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
                name: "Acme Sensor".into(),
                device_id: MacAddress::new([10, 20, 30, 40, 50, 63]),
                category: AccessoryCategory::Sensor,
                ..Default::default()
            };
            storage.save_config(&config).await?;
            config
        },
    };

    let server = IpServer::new(config, storage).await?;
    let sensor_ptr = server.add_accessory(sensor).await?;

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

            motion_detected_characteristic.set_value(Value::Bool(true)).await?;

            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            motion_detected_characteristic.set_value(Value::Bool(false)).await?;
        }

        #[allow(unreachable_code)]
        Ok(())
    };

    std::env::set_var("RUST_LOG", "hap=debug");
    env_logger::init();

    futures::try_join!(handle, value_set_interval)?;

    Ok(())
}
