use serde_json;

use service::HapService;

pub mod outlet;

pub trait HapAccessory {
    fn get_id(&self) -> &u64;
    fn set_id(&mut self, id: u64);
    fn get_services(&self) -> Vec<&HapService>;
    fn get_mut_services(&mut self) -> Vec<&mut HapService>;
    fn set_information(&mut self, information: Information);
    fn to_json(&self) -> serde_json::Value;
}

pub fn init_iids(accessory: &mut Box<HapAccessory>) {
    let mut next_iid = 1;
    for service in accessory.get_mut_services() {
        service.set_id(next_iid);
        next_iid += 1;
        for characteristic in service.get_mut_characteristics() {
            characteristic.set_id(next_iid);
            next_iid += 1;
        }
    }
}

// TODO - maybe infere the types somehow from characteristic::Characteristic
pub struct Information {
    pub identify: bool,
    pub manufacturer: String,
    pub model: String,
    pub name: String,
    pub serial_number: String,
    pub firmware_revision: String,
}

impl Default for Information {
    fn default() -> Information {
        Information {
            identify: false,
            manufacturer: "undefined".into(),
            model: "undefined".into(),
            name: "undefined".into(),
            serial_number: "undefined".into(),
            firmware_revision: "undefined".into(),
        }
    }
}

pub enum Category {
    Other,
    Bridge,
    Fan,
    Garage,
    Lightbulb,
    DoorLock,
    Outlet,
    Switch,
    Thermostat,
    Sensor,
    SecuritySystem,
    Door,
    Window,
    WindowCovering,
    ProgrammableSwitch,
    RangeExtender,
    IPCamera,
    VideoDoorBell,
    AirPurifier,
}

impl Category {
    pub fn as_u8(&self) -> u8 {
        match self {
            &Category::Other => 1,
            &Category::Bridge => 2,
            &Category::Fan => 3,
            &Category::Garage => 4,
            &Category::Lightbulb => 5,
            &Category::DoorLock => 6,
            &Category::Outlet => 7,
            &Category::Switch => 8,
            &Category::Thermostat => 9,
            &Category::Sensor => 10,
            &Category::SecuritySystem => 11,
            &Category::Door => 12,
            &Category::Window => 13,
            &Category::WindowCovering => 14,
            &Category::ProgrammableSwitch => 15,
            &Category::RangeExtender => 16,
            &Category::IPCamera => 17,
            &Category::VideoDoorBell => 18,
            &Category::AirPurifier => 19,
        }
    }
}
