extern crate hwaddr;

use std::net::IpAddr;
use hwaddr::HwAddr;

pub struct Config {
    pub storage_path: String,
    pub port: int,
    pub ip: IpAddr,
    pub pin: String,
    pub name: String,
    pub id: HwAddr,
}
