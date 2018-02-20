use serde::ser::{Serialize, Serializer, SerializeStruct};
use erased_serde;

use service::HapService;
use transport::accessory_list::AccessoryListTrait;

pub mod outlet;

trait HapAccessoryService: HapService + erased_serde::Serialize {}

impl<T: HapService + erased_serde::Serialize> HapAccessoryService for T {}

serialize_trait_object!(HapAccessoryService);

pub trait HapAccessory {
    fn get_id(&self) -> u64;
    fn set_id(&mut self, id: u64);
    fn get_services(&self) -> Vec<&HapAccessoryService>;
    fn get_mut_services(&mut self) -> Vec<&mut HapAccessoryService>;
}

pub struct Accessory<T: HapAccessory> {
    inner: T,
}

impl<T: HapAccessory> Accessory<T> {
    fn new(inner: T) -> Accessory<T> {
        Accessory { inner }
    }
}

impl<T: HapAccessory> Serialize for Accessory<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}

impl<T: HapAccessory> HapAccessory for Accessory<T> {
    fn get_id(&self) -> u64 {
        self.inner.get_id()
    }

    fn set_id(&mut self, id: u64) {
        self.inner.set_id(id)
    }

    fn get_services(&self) -> Vec<&HapAccessoryService> {
        self.inner.get_services()
    }

    fn get_mut_services(&mut self) -> Vec<&mut HapAccessoryService> {
        self.inner.get_mut_services()
    }
}

pub fn init_iids(accessory: &mut Box<AccessoryListTrait>) {
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
