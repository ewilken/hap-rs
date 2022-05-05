use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

use uuid::Uuid;

use hap::{
    accessory::{AccessoryCategory, AccessoryInformation, HapAccessory},
    characteristic::{Characteristic, Format, HapCharacteristic, Perm},
    server::{IpServer, Server},
    service::{accessory_information::AccessoryInformationService, HapService},
    storage::{FileStorage, Storage},
    Config,
    HapType,
    MacAddress,
    Pin,
    Result,
};

// creating a custom service

/// Foo service.
#[derive(Debug, Default)]
pub struct FooService {
    /// Instance ID of the Foo service.
    id: u64,
    /// [`HapType`](HapType) of the Foo service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

    /// Foo Number characteristic (required).
    pub foo_number: Characteristic<u8>,
    /// Foo Name characteristic (required).
    pub foo_name: Characteristic<String>,
}

impl HapService for FooService {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_type(&self) -> HapType { self.hap_type }

    fn set_type(&mut self, hap_type: HapType) { self.hap_type = hap_type; }

    fn get_hidden(&self) -> bool { self.hidden }

    fn set_hidden(&mut self, hidden: bool) { self.hidden = hidden; }

    fn get_primary(&self) -> bool { self.primary }

    fn set_primary(&mut self, primary: bool) { self.primary = primary; }

    fn get_linked_services(&self) -> Vec<u64> { self.linked_services.clone() }

    fn set_linked_services(&mut self, linked_services: Vec<u64>) { self.linked_services = linked_services; }

    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
        for characteristic in self.get_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
        for characteristic in self.get_mut_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> { vec![&self.foo_number, &self.foo_name] }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        vec![&mut self.foo_number, &mut self.foo_name]
    }
}

impl serde::ser::Serialize for FooService {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}

// creating a custom accessory

/// Foo accessory.
#[derive(Debug, Default)]
pub struct FooAccessory {
    /// ID of the Foo accessory.
    id: u64,

    /// Accessory Information service.
    pub accessory_information: AccessoryInformationService,
    /// Foo service.
    pub foo: FooService,
}

impl HapAccessory for FooAccessory {
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

    fn get_services(&self) -> Vec<&dyn HapService> { vec![&self.accessory_information, &self.foo] }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> { vec![&mut self.accessory_information, &mut self.foo] }
}

impl Serialize for FooAccessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Custom Foo Number characteristic.
    let foo_number = Characteristic::<u8>::new(
        8,
        1,
        HapType::Custom(Uuid::parse_str("2db3ac3f-8b9c-4431-8d87-670351dc872a").unwrap()),
        Format::UInt8,
        vec![Perm::PairedRead],
        Some("Foo Number Characteristic".to_string()),
        None,
        42,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    // Custom Foo Name characteristic.
    let foo_name = Characteristic::<String>::new(
        9,
        1,
        HapType::Custom(Uuid::parse_str("497c968e-261f-445d-bcac-69ae7bb8979b").unwrap()),
        Format::String,
        vec![Perm::PairedRead],
        Some("Foo Name Characteristic".to_string()),
        None,
        "Horst".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    // Custom Foo service.
    let foo_service = FooService {
        id: 7, // accessory information service ends at IID 6, so we start counting at 7
        hap_type: HapType::Custom(Uuid::parse_str("d5bb0a60-92f0-483e-811d-97d4b2b502ff").unwrap()),
        hidden: false,
        primary: true,
        linked_services: vec![],
        foo_number,
        foo_name,
    };

    // Custom Foo accessory.
    let mut foo_accessory = FooAccessory {
        id: 1,
        accessory_information: AccessoryInformation {
            name: "Acme Foo".into(),
            ..Default::default()
        }
        .to_service(1, 1)?,
        foo: foo_service,
    };

    foo_accessory.foo.foo_number.on_read(Some(|| {
        println!("foo_number characteristic read");
        Ok(None)
    }));

    foo_accessory.foo.foo_name.on_read(Some(|| {
        println!("foo_name characteristic read");
        Ok(None)
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
                name: "Acme Foo".into(),
                device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
                category: AccessoryCategory::Other,
                ..Default::default()
            };
            storage.save_config(&config).await?;
            config
        },
    };

    let server = IpServer::new(config, storage).await?;
    server.add_accessory(foo_accessory).await?;

    let handle = server.run_handle();

    std::env::set_var("RUST_LOG", "hap=debug");
    env_logger::init();

    handle.await
}
