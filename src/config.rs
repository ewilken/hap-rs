use std::net::{IpAddr, Ipv4Addr};
use eui48::MacAddress;
use rand;
use rand::Rng;
use serde_json;

use accessory::Category;

use bonjour::{StatusFlag, FeatureFlag};

pub struct Config {
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
}

impl Config {
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
            storage_path: "/tmp/".into(),
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
        }
    }
}

fn new_random_mac_address() -> MacAddress {
    let mut rng = rand::thread_rng();
    let eui = [
        rng.gen::<u8>(),
        rng.gen::<u8>(),
        rng.gen::<u8>(),
        rng.gen::<u8>(),
        rng.gen::<u8>(),
        rng.gen::<u8>(),
    ];
    MacAddress::new(eui)
}
