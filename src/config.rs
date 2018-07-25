use std::{
    net::IpAddr,
    env::current_dir,
    str,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    rc::Rc,
    cell::RefCell,
};

use eui48::MacAddress;
use rand::{self, Rng};
use uuid::Uuid;
use pnet::datalink;

use accessory::Category;
use db::Storage;
use transport::bonjour::{StatusFlag, FeatureFlag};

use Error;

pub type ConfigPtr = Rc<RefCell<Config>>;

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
    pub max_peers: Option<usize>,
    pub config_hash: Option<u64>,
}

impl Config {
    pub fn load(&mut self, storage: &Storage) {
        if let Some(device_id) = storage.get_byte_vec("device_id").ok() {
            // TODO - make this less shitty
            self.device_id = MacAddress::parse_str(str::from_utf8(&device_id).unwrap()).unwrap();
        }
        if let Some(version) = storage.get_u64("version").ok() {
            self.version = version;
        }
        if let Some(config_hash) = storage.get_u64("config_hash").ok() {
            self.config_hash = Some(config_hash);
        }
    }

    pub fn save(&self, storage: &Storage) -> Result<(), Error> {
        storage.set_byte_vec("device_id", self.device_id.to_hex_string().as_bytes().to_vec())?;
        storage.set_u64("version", self.version.clone())?;
        if let Some(config_hash) = self.config_hash {
            storage.set_u64("config_hash", config_hash)?;
        }
        Ok(())
    }

    fn calculate_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }

    fn set_hash(&mut self, config_hash: u64) {
        if self.config_hash != Some(config_hash) {
            self.version += 1;
            self.config_hash = Some(config_hash);
        }
    }

    pub fn update_hash(&mut self) {
        let hash = self.calculate_hash();
        self.set_hash(hash);
    }

    pub fn txt_records(&self) -> [String; 8] {
        [
            format!("md={}", self.name),
            format!("id={}", self.device_id.to_hex_string()),
            format!("c#={}", self.configuration_number),
            format!("s#={}", self.state_number),
            format!("ci={}", self.category as u8),
            format!("pv={}", self.protocol_version),
            format!("sf={}", self.status_flag as u8),
            format!("ff={}", self.feature_flag as u8),
        ]
    }
}

impl Hash for Config {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.storage_path.hash(state);
        self.port.hash(state);
        self.ip.hash(state);
        self.pin.hash(state);
        self.name.hash(state);
        self.device_id.to_hex_string().hash(state);
        self.configuration_number.hash(state);
        self.state_number.hash(state);
        (self.category as u8).hash(state);
        self.protocol_version.hash(state);
        (self.status_flag as u8).hash(state);
        (self.feature_flag as u8).hash(state);
    }
}

impl Default for Config {
    fn default() -> Config {
        let mut config = Config {
            id: Uuid::new_v4(),
            version: 0,
            storage_path: format!("{}/data", current_dir().unwrap().to_str().unwrap()),
            port: 32000,
            ip: current_ip().expect("could not determine local IP address"),
            pin: "11122333".into(),
            name: "Accessory".into(),
            device_id: random_mac_address(),
            configuration_number: 1,
            state_number: 1,
            category: Category::Unknown,
            protocol_version: "1.0".into(),
            status_flag: StatusFlag::NotPaired,
            feature_flag: FeatureFlag::Zero,
            max_peers: None,
            config_hash: None,
        };
        config.update_hash();
        config
    }
}

fn current_ip() -> Option<IpAddr> {
    for iface in datalink::interfaces() {
        for ip_network in iface.ips {
            if ip_network.is_ipv4() {
                let ip = ip_network.ip();
                if !ip.is_loopback() {
                    return Some(ip);
                }
            }
        }
    }
    None
}

fn random_mac_address() -> MacAddress {
    let mut rng = rand::thread_rng();
    let eui = rng.gen::<[u8; 6]>();
    MacAddress::new(eui)
}
