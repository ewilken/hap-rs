use service;

pub mod outlet;

pub trait AccessoryT {
    fn get_services(&self) -> Vec<&service::ServiceT>;
    fn set_information(&mut self, information: Information);
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
