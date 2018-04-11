use serde::ser::{Serialize, Serializer, SerializeStruct};
use erased_serde;

use service::{HapService, accessory_information::AccessoryInformation};
use db::accessory_list::AccessoryListTrait;

pub mod outlet;

pub trait HapAccessoryService: HapService + erased_serde::Serialize {}

impl<T: HapService + erased_serde::Serialize> HapAccessoryService for T {}

serialize_trait_object!(HapAccessoryService);

pub trait HapAccessory {
    fn get_id(&self) -> u64;
    fn set_id(&mut self, id: u64);
    fn get_services(&self) -> Vec<&HapAccessoryService>;
    fn get_mut_services(&mut self) -> Vec<&mut HapAccessoryService>;
    fn get_mut_information(&mut self) -> &mut AccessoryInformation;
}

pub struct Accessory<T: HapAccessory> {
    pub inner: T,
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

    fn get_mut_information(&mut self) -> &mut AccessoryInformation {
        self.inner.get_mut_information()
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

#[derive(Copy, Clone)]
pub enum Category {
    Other = 1,
    Bridge = 2,
    Fan = 3,
    Garage = 4,
    Lightbulb = 5,
    DoorLock = 6,
    Outlet = 7,
    Switch = 8,
    Thermostat = 9,
    Sensor = 10,
    SecuritySystem = 11,
    Door = 12,
    Window = 13,
    WindowCovering = 14,
    ProgrammableSwitch = 15,
    RangeExtender = 16,
    IPCamera = 17,
    VideoDoorBell = 18,
    AirPurifier = 19,
}
