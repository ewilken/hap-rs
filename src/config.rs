use ed25519_dalek::Keypair as Ed25519Keypair;
//use eui48::MacAddress;
use macaddr::MacAddr6 as MacAddress;
use rand::{rngs::OsRng, Rng};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

use crate::{accessory::AccessoryCategory, BonjourFeatureFlag, BonjourStatusFlag, Pin};

/// The `Config` struct is used to store configuration options for the HomeKit Accessory Server.
///
/// # Examples
///
/// ```
/// use hap::{accessory::AccessoryCategory, Config, MacAddress, Pin};
///
/// let config = Config {
///     pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap(),
///     name: "Acme Lightbulb".into(),
///     device_id: MacAddress::from([10, 20, 30, 40, 50, 60]),
///     category: AccessoryCategory::Lightbulb,
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Socket IP address to serve on. Defaults to the IP of the system's first non-loopback network interface.
    pub host: IpAddr,
    /// Port to serve on. Defaults to `32000`.
    pub port: u16,
    /// 8 digit pin used for pairing. Defaults to `11122333`.
    ///
    /// The following pins are considered too easy and are therefore not allowed:
    /// - `12345678`
    /// - `87654321`
    /// - `00000000`
    /// - `11111111`
    /// - `22222222`
    /// - `33333333`
    /// - `44444444`
    /// - `55555555`
    /// - `66666666`
    /// - `77777777`
    /// - `88888888`
    /// - `99999999`
    pub pin: Pin,
    /// Model name of the accessory. E.g. "Acme Lightbulb".
    pub name: String,
    /// Device ID of the accessory. Generated randomly if not specified. This value is also used as the accessory's
    /// Pairing Identifier. Must be a unique random number generated at every factory reset and must persist across
    /// reboots.
    pub device_id: MacAddress, // Bonjour: id
    ///
    pub device_ed25519_keypair: Ed25519Keypair,
    /// Current configuration number. Is updated when an accessory, service, or characteristic is added or removed on
    /// the accessory server. Accessories must increment the config number after a firmware update.
    pub configuration_number: u64, // Bonjour: c#
    /// Current state number. This must have a value of `1`.
    pub state_number: u8, // Bonjour: s#
    /// Accessory category. Indicates the category that best describes the primary function of the accessory.
    pub category: AccessoryCategory, // Bonjour: ci
    /// Protocol version string `<major>.<minor>` (e.g. `"1.0"`). Defaults to `"1.0"` Required if value is not `"1.0"`.
    pub protocol_version: String, // Bonjour: pv
    /// Bonjour Status Flag. Defaults to `StatusFlag::NotPaired` and is changed to `StatusFlag::Zero` after a
    /// successful pairing.
    pub status_flag: BonjourStatusFlag, // Bonjour: sf
    /// Bonjour Feature Flag. Currently only used to indicate MFi compliance.
    pub feature_flag: BonjourFeatureFlag, // Bonjour: ff
    /// Optional maximum number of paired controllers.
    pub max_peers: Option<usize>,
}

impl Config {
    /// Redetermines the `host` field to the IP of the system's first non-loopback network interface.
    pub fn redetermine_local_ip(&mut self) {
        self.host = get_local_ip();
    }

    /// Derives mDNS TXT records from the `Config`.
    pub(crate) fn txt_records(&self) -> [String; 8] {
        [
            format!("c#={}", self.configuration_number),
            format!("ff={}", self.feature_flag as u8),
            format!("id={}", self.device_id.to_string()),
            format!("md={}", self.name),
            format!("pv={}", self.protocol_version),
            format!("s#={}", self.state_number),
            format!("sf={}", self.status_flag as u8),
            format!("ci={}", self.category as u8),
            // format!("sh={}", self.setup_hash as u8), setup hash seems to be still undocumented
        ]
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            host: get_local_ip(),
            port: 32000,
            pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap(),
            name: "Accessory".into(),
            device_id: generate_random_mac_address(),
            device_ed25519_keypair: generate_ed25519_keypair(),
            configuration_number: 1,
            state_number: 1,
            category: AccessoryCategory::Other,
            protocol_version: "1.0".into(),
            status_flag: BonjourStatusFlag::NotPaired,
            feature_flag: BonjourFeatureFlag::Zero,
            max_peers: None,
        }
    }
}

/// Generates a random MAC address.
fn generate_random_mac_address() -> MacAddress {
    let mut csprng = OsRng {};
    let eui = csprng.gen::<[u8; 6]>();
    MacAddress::from(eui)
}

/// Generates an Ed25519 keypair.
fn generate_ed25519_keypair() -> Ed25519Keypair {
    let mut csprng = OsRng {};
    Ed25519Keypair::generate(&mut csprng)
}

/// Returns the IP of the system's first non-loopback network interface or defaults to `127.0.0.1`.
fn get_local_ip() -> IpAddr {
    for iface in if_addrs::get_if_addrs().unwrap() {
        if !iface.is_loopback() {
            return iface.ip();
        }
    }
    "127.0.0.1".parse().unwrap()
}
