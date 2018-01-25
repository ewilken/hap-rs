use std::net::{IpAddr, Ipv4Addr};
use std::io::Error;
use eui48::MacAddress;
use rand;
use rand::Rng;
use uuid::Uuid;

use accessory::Category;

use db::storage::Storage;
use transport::bonjour::{StatusFlag, FeatureFlag};

pub struct Config {
    pub id: Uuid,
    pub version: u64,
    pub storage_path: String,
    pub port: u16,
    pub ip: IpAddr,
    pub pin: String,
    pub name: String,              // md
    pub device_id: MacAddress,     // id
    pub configuration_number: u64, // c#
    pub state_number: u8,          // s#
    pub category: Category,        // ci
    pub protocol_version: String,  // pv
    pub status_flag: StatusFlag,   // sf
    pub feature_flag: FeatureFlag, // ff
    pub config_hash: u64,
}

impl Config {
    pub fn load(&mut self, storage: &Storage) {
        if let Some(id) = storage.get_uuid("uuid").ok() {
            self.id = id;
        }
        if let Some(version) = storage.get_u64("version").ok() {
            self.version = version;
        }
        if let Some(config_hash) = storage.get_u64("config_hash").ok() {
            self.config_hash = config_hash;
        }
    }

    pub fn save(&self, storage: &Storage) -> Result<(), Error> {
        storage.set_uuid("uuid", self.id.to_owned())?;
        storage.set_u64("version", self.version.to_owned())?;
        storage.set_u64("config_hash", self.config_hash.to_owned())?;
        Ok(())
    }

    pub fn update_hash(&mut self, config_hash: u64) {
        if self.config_hash != config_hash {
            self.version += 1;
            self.config_hash = config_hash;
        }
    }

    fn default_storage_path(&self) -> String {
        self.name.to_owned()
    }

    pub fn txt_records(&self) -> [String; 8] {
        [
            format!("md={}", self.name),
            format!("id={}", self.device_id.to_hex_string()),
            format!("c#={}", self.configuration_number),
            format!("s#={}", self.state_number),
            format!("ci={}", self.category.as_u8()),
            format!("pv={}", self.protocol_version),
            format!("sf={}", self.status_flag.as_u8()),
            format!("ff={}", self.feature_flag.as_u8()),
        ]
    }
}

// TODO - add default values that actually make sense
impl Default for Config {
    fn default() -> Config {
        Config {
            id: Uuid::new_v4(),
            version: 0,
            // TODO - default storage path should be == name automatically
            storage_path: "/tmp/Accessory".into(),
            port: 32000,
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            pin: "11122333".into(),
            name: "Accessory".into(),
            device_id: random_mac_address(),
            configuration_number: 1,
            state_number: 1,
            // TODO - default category should probably be Switch
            category: Category::Outlet,
            protocol_version: "1.0".into(),
            status_flag: StatusFlag::NotPaired,
            feature_flag: FeatureFlag::Zero,
            config_hash: 0,
        }
    }
}

fn random_mac_address() -> MacAddress {
    let mut rng = rand::thread_rng();
    let eui = rng.gen::<[u8; 6]>();
    MacAddress::new(eui)
}
