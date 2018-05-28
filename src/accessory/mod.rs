use serde::ser::{Serialize, Serializer, SerializeStruct};
use erased_serde;

use service::{HapService, accessory_information::{self, AccessoryInformation}};
use characteristic::{hardware_revision, accessory_flags};
use event::EmitterPtr;

pub mod outlet;

mod category;
pub use accessory::category::Category;

pub trait HapAccessoryService: HapService + erased_serde::Serialize {}

impl<T: HapService + erased_serde::Serialize> HapAccessoryService for T {}

serialize_trait_object!(HapAccessoryService);

pub trait HapAccessory {
    fn get_id(&self) -> u64;
    fn set_id(&mut self, id: u64);
    fn get_services(&self) -> Vec<&HapAccessoryService>;
    fn get_mut_services(&mut self) -> Vec<&mut HapAccessoryService>;
    fn get_mut_information(&mut self) -> &mut AccessoryInformation;
    fn init_iids(&mut self, accessory_id: u64, event_emitter: EmitterPtr);
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

    fn init_iids(&mut self, accessory_id: u64, event_emitter: EmitterPtr) {
        self.inner.init_iids(accessory_id, event_emitter)
    }
}

pub struct Information {
    pub identify: bool,
    pub manufacturer: String,
    pub model: String,
    pub name: String,
    pub serial_number: String,
    pub firmware_revision: String,
    pub hardware_revision: Option<String>,
	pub accessory_flags: Option<u32>,
}

impl Information {
    pub fn to_service(self) -> AccessoryInformation {
        let mut i = accessory_information::new();
        i.inner.identify.set_value(self.identify).unwrap();
        i.inner.manufacturer.set_value(self.manufacturer).unwrap();
        i.inner.model.set_value(self.model).unwrap();
        i.inner.name.set_value(self.name).unwrap();
        i.inner.serial_number.set_value(self.serial_number).unwrap();
        i.inner.firmware_revision.set_value(self.firmware_revision).unwrap();
        if let Some(v) = self.hardware_revision {
            let mut hr = hardware_revision::new();
            hr.set_value(v).unwrap();
            i.inner.hardware_revision = Some(hr);
        }
        if let Some(v) = self.accessory_flags {
            let mut af = accessory_flags::new();
            af.set_value(v).unwrap();
            i.inner.accessory_flags = Some(af);
        }
        i
    }
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
            hardware_revision: None,
            accessory_flags: None,
        }
    }
}
