use std::net::{IpAddr, Ipv4Addr};
use std::io::Error;
use eui48::MacAddress;
use rand;
use rand::Rng;
use serde_json;
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
    fn load(&mut self, storage: &Storage) {
        if let Some(id) = storage.get("uuid").ok() {
            self.id = id;
        }
        if let Some(version) = storage.get("version").ok() {
            self.version = version;
        }
        if let Some(config_hash) = storage.get("config_hash").ok() {
            self.config_hash = config_hash;
        }
    }

    fn save(&self, storage: &Storage) -> Result<(), Error> {
        storage.set("uuid", self.id.as_bytes().to_vec())?;
        storage.set("version", self.version)?;
        storage.set("config_hash", self.config_hash)?;
        Ok(())
    }

    fn update_hash(&mut self, config_hash: u64) {
        if self.config_hash != config_hash {
            self.version += 1;
            self.config_hash = config_hash;
        }
    }

    fn as_txt_records(&self) -> serde_json::Value {
        json!({
            "pv": self.protocol_version,
    		"id": self.device_id.to_hex_string(),
    		"c#": self.configuration_number,
    		"s#": self.state_number,
    		"sf": self.status_flag.as_u8(),
    		"ff": self.feature_flag.as_u8(),
    		"md": self.name,
    		"ci": self.category.as_u8(),
        })
    }
}

// TODO - add default values that actually make sense
impl Default for Config {
    fn default() -> Config {
        Config {
            id: Uuid::new_v4(),
            version: 0,
            // TODO - default storage path should be == name automatically
            storage_path: "Accessory".into(),
            port: 32000,
            ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            pin: "00102003".into(),
            name: "Accessory".into(),
            device_id: new_random_mac_address(),
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

fn new_random_mac_address() -> MacAddress {
    let mut rng = rand::thread_rng();
    let eui = rng.gen::<[u8; 6]>();
    MacAddress::new(eui)
}
