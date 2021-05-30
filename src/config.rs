use ed25519_dalek::Keypair as Ed25519Keypair;
use eui48::MacAddress;
use rand::{rngs::OsRng, Rng};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

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
///     device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
///     category: AccessoryCategory::Lightbulb,
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Socket address to serve on.
    pub socket_addr: SocketAddr,
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
    /// Pairing Identifier.
    pub device_id: MacAddress, // id
    ///
    pub device_ed25519_keypair: Ed25519Keypair,
    /// Current configuration number. Is updated when an accessory, service, or characteristic is added or removed on
    /// the accessory server. Accessories must increment the config number after a firmware update.
    pub configuration_number: u64, // c#
    /// Current state number. This must have a value of `1`.
    pub state_number: u8, // s#
    /// Accessory category. Indicates the category that best describes the primary function of the accessory.
    pub category: AccessoryCategory, // ci
    /// Protocol version string `<major>.<minor>` (e.g. `"1.0"`). Defaults to `"1.0"` Required if value is not `"1.0"`.
    pub protocol_version: String, // pv
    /// Bonjour Status Flag. Defaults to `StatusFlag::NotPaired` and is changed to `StatusFlag::Zero` after a
    /// successful pairing.
    pub status_flag: BonjourStatusFlag, // sf
    /// Bonjour Feature Flag. Currently only used to indicate MFi compliance.
    pub feature_flag: BonjourFeatureFlag, // ff
    /// Optional maximum number of paired controllers.
    pub max_peers: Option<usize>,
}

impl Config {
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

impl Default for Config {
    fn default() -> Config {
        Config {
            socket_addr: get_current_ipv4(),
            pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap(),
            name: "Accessory".into(),
            device_id: generate_random_mac_address(),
            device_ed25519_keypair: generate_ed25519_keypair(),
            configuration_number: 1,
            state_number: 1,
            category: AccessoryCategory::Unknown,
            protocol_version: "1.0".into(),
            status_flag: BonjourStatusFlag::NotPaired,
            feature_flag: BonjourFeatureFlag::Zero,
            max_peers: None,
        }
    }
}

fn generate_random_mac_address() -> MacAddress {
    let mut csprng = OsRng {};
    let eui = csprng.gen::<[u8; 6]>();
    MacAddress::new(eui)
}

fn generate_ed25519_keypair() -> Ed25519Keypair {
    let mut csprng = OsRng {};
    Ed25519Keypair::generate(&mut csprng)
}

fn get_current_ipv4() -> SocketAddr {
    let socket = match std::net::UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return SocketAddr::from(([127, 0, 0, 1], 32000)),
    };

    match socket.connect("8.8.8.8:80") {
        Ok(_) => {},
        Err(_) => return SocketAddr::from(([127, 0, 0, 1], 32000)),
    }

    match socket.local_addr() {
        Ok(a) => SocketAddr::from((a.ip(), 32000)),
        Err(_) => return SocketAddr::from(([127, 0, 0, 1], 32000)),
    }
}
