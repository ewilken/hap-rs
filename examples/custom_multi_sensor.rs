use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

use hap::{
    accessory::{AccessoryCategory, AccessoryInformation, HapAccessory},
    server::{IpServer, Server},
    service::{
        accessory_information::AccessoryInformationService,
        humidity_sensor::HumiditySensorService,
        temperature_sensor::TemperatureSensorService,
        HapService,
    },
    storage::{FileStorage, Storage},
    Config,
    HapType,
    MacAddress,
    Pin,
    Result,
};

/// Multi Sensor accessory.
#[derive(Debug, Default)]
pub struct MultiSensorAccessory {
    /// ID of the Multi Sensor accessory.
    id: u64,

    /// Accessory Information service.
    pub accessory_information: AccessoryInformationService,
    /// Temperature Sensor service.
    pub temperature_sensor: TemperatureSensorService,
    /// Humidity Sensor service.
    pub humidity_sensor: HumiditySensorService,
}

impl HapAccessory for MultiSensorAccessory {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_service(&self, hap_type: HapType) -> Option<&dyn HapService> {
        for service in self.get_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_mut_service(&mut self, hap_type: HapType) -> Option<&mut dyn HapService> {
        for service in self.get_mut_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_services(&self) -> Vec<&dyn HapService> {
        vec![
            &self.accessory_information,
            &self.temperature_sensor,
            &self.humidity_sensor,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
        vec![
            &mut self.accessory_information,
            &mut self.temperature_sensor,
            &mut self.humidity_sensor,
        ]
    }
}

impl Serialize for MultiSensorAccessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let multi_sensor = MultiSensorAccessory {
        id: 1,
        accessory_information: AccessoryInformation {
            name: "Acme Temperature & Humidity Sensor".into(),
            ..Default::default()
        }
        .to_service(1, 1)?,
        // accessory information service ends at IID 6, so we start counting at 7
        temperature_sensor: TemperatureSensorService::new(7, 1),
        // teperature sensor service ends at IID 13, so we start counting at 14
        humidity_sensor: HumiditySensorService::new(14, 1),
    };

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
                name: "Acme Temperature & Humidity Sensor".into(),
                device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
                category: AccessoryCategory::Sensor,
                ..Default::default()
            };
            storage.save_config(&config).await?;
            config
        },
    };

    let server = IpServer::new(config, storage).await?;
    server.add_accessory(multi_sensor).await?;

    let handle = server.run_handle();

    std::env::set_var("RUST_LOG", "hap=debug");
    env_logger::init();

    handle.await
}
