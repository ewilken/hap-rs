use serde_json;

use service::Service;

pub mod outlet;

#[derive(Default)]
pub struct Accessory {
    pub id: u64,
    pub services: Vec<Service>,
}

impl Accessory {
    /*fn set_information(&mut self, information: Information) {
        self.services[0].characteristics[0].set_value(information.identify).unwrap();
        self.services[0].characteristics[1].set_value(information.manufacturer).unwrap();
        self.services[0].characteristics[2].set_value(information.model).unwrap();
        self.services[0].characteristics[3].set_value(information.name).unwrap();
        self.services[0].characteristics[4].set_value(information.serial_number).unwrap();
        self.services[0].characteristics[5].set_value(information.firmware_revision).unwrap();
    }*/

    pub fn as_json(&self) -> serde_json::Value {
        let services: Vec<serde_json::Value> = self.services.iter().map(|s| s.as_json()).collect();
        json!({
            "aid": self.id,
            "services": services,
        })
    }
}

// TODO - maybe infere the types somehow from characteristic::Characteristic
#[derive(Default)]
pub struct Information {
    pub identify: bool,
    pub manufacturer: String,
    pub model: String,
    pub name: String,
    pub serial_number: String,
    pub firmware_revision: String,
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
