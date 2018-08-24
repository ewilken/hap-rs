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
use pnet::datalink;

use accessory::Category;
use db::Storage;
use transport::bonjour::{StatusFlag, FeatureFlag};

use Error;

/// Reference counting pointer to a `Config`.
pub type ConfigPtr = Rc<RefCell<Config>>;

/// The `Config` struct is used to store configuration options for the HomeKit Accessory Server.
///
/// # Examples
///
/// ```
/// let config = Config {
///     storage_path: "/etc/homekit".into(),
///     pin: "11122333".into(),
///     name: "Acme Outlet".into(),
///     category: Category::Outlet,
///     max_peers: Some(32),
///     ..Default::default()
/// };
/// ```
pub struct Config {
    /// Storage path for the persisted data. If no path is specified, the current working directory
    /// is used.
    pub storage_path: String,
    /// IP address to serve on.
    pub ip: IpAddr,
    /// Port to serve on. Defaults to `32000`.
    pub port: u16,
    /// 8 digit pin used for pairing. Defaults to `"11122333"`.
    ///
    /// The following pins are considered too easy by Apple and therefore not allowed:
    /// - `"12345678"`
    /// - `"87654321"`
    /// - `"00000000"`
    /// - `"11111111"`
    /// - `"22222222"`
    /// - `"33333333"`
    /// - `"44444444"`
    /// - `"55555555"`
    /// - `"66666666"`
    /// - `"77777777"`
    /// - `"88888888"`
    /// - `"99999999"`
    pub pin: String,
    /// Model name of the accessory.
    pub name: String,
    /// Device ID of the accessory. Generated randomly if not specified. This value is also used as
    /// the accessory's Pairing Identifier.
    pub device_id: MacAddress, // id
    /// Current configuration number. Is updated when an accessory, service, or characteristic is
    /// added or removed on the accessory server. Accessories must increment the config number after
    /// a firmware update.
    pub configuration_number: u64, // c#
    /// Current state number. This must have a value of `1`.
    pub state_number: u8, // s#
    /// Accessory Category. Indicates the category that best describes the primary function of the
    /// accessory.
    pub category: Category, // ci
    /// Protocol version string `<major>.<minor>` (e.g. `"1.0"`). Defaults to `"1.0"` Required if value
    /// is not `"1.0"`.
    pub protocol_version: String, // pv
    /// Bonjour Status Flag. Defaults to `StatusFlag::NotPaired` and is changed to
    /// `StatusFlag::Zero` after a successful pairing.
    pub status_flag: StatusFlag, // sf
    /// Bonjour Feature Flag. Currently only used to indicate MFi compliance.
    pub feature_flag: FeatureFlag, // ff
    /// Optional maximum number of paired controllers.
    pub max_peers: Option<usize>,
    pub version: u64,
    pub config_hash: Option<u64>,
}

impl Config {
    pub(crate) fn load_from(&mut self, storage: &Storage) -> Result<(), Error> {
        if let Some(device_id) = storage.get_bytes("device_id").ok() {
            self.device_id = MacAddress::parse_str(str::from_utf8(&device_id)?)?;
        }
        if let Some(version) = storage.get_u64("version").ok() {
            self.version = version;
        }
        if let Some(config_hash) = storage.get_u64("config_hash").ok() {
            self.config_hash = Some(config_hash);
        }
        Ok(())
    }

    pub(crate) fn save_to(&self, storage: &Storage) -> Result<(), Error> {
        storage.set_bytes("device_id", self.device_id.to_hex_string().as_bytes().to_vec())?;
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

    pub(crate) fn update_hash(&mut self) {
        let hash = self.calculate_hash();
        self.set_hash(hash);
    }

    pub(crate) fn txt_records(&self) -> [String; 8] {
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
        self.storage_path.hash(state);
        self.ip.hash(state);
        self.port.hash(state);
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
            storage_path: format!(
                "{}/data", current_dir()
                    .expect("couldn't determine current directory")
                    .to_str()
                    .expect("couldn't stringify current directory")
            ),
            ip: current_ip().expect("couldn't determine local IP address"),
            port: 32000,
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
            version: 0,
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
