use erased_serde::{self, serialize_trait_object};
use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    characteristic::{accessory_flags, hardware_revision},
    pointer,
    service::{
        accessory_information::{self, AccessoryInformation},
        HapService,
    },
    Result,
};

mod category;
mod defined;
mod generated;

pub use crate::accessory::{category::Category, defined::*, generated::*};

/// `HapAccessoryService` is implemented by every `Service` inside of an `Accessory`.
pub trait HapAccessoryService: HapService + erased_serde::Serialize {}

impl<T: HapService + erased_serde::Serialize> HapAccessoryService for T {}

serialize_trait_object!(HapAccessoryService);

/// `HapAccessory` is implemented by the inner type of every `Accessory`.
pub trait HapAccessory {
    /// Returns the ID of an Accessory.
    fn get_id(&self) -> u64;
    /// Sets the ID of an Accessory.
    fn set_id(&mut self, id: u64);
    /// Returns references to all Services of an Accessory.
    fn get_services(&self) -> Vec<&dyn HapAccessoryService>;
    /// Returns mutable references to the Services of an Accessory.
    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService>;
    /// Returns a mutable reference to the Accessory Information Service of an Accessory.
    fn get_mut_information(&mut self) -> &mut AccessoryInformation;
    /// Initializes the Service and Characteristic instance IDs of an Accessory. Service and
    /// Characteristic instance IDs, "iid", are assigned from the same number pool that is unique
    /// within each Accessory object. For example, if the first Service object has an instance ID of
    /// "1" then no other Service or Characteristic objects can have an instance ID of "1" within
    /// the parent Accessory object.
    fn init_iids(&mut self, accessory_id: u64, event_emitter: pointer::EventEmitter) -> Result<()>;
}

/// An Accessory. Accessories are the outermost data type defined by the HAP. They are comprised of
/// services and characteristics.
pub struct Accessory<T: HapAccessory> {
    pub inner: T,
}

impl<T: HapAccessory> Accessory<T> {
    /// Creates a new `Accessory`.
    fn new(inner: T) -> Accessory<T> { Accessory { inner } }
}

impl<T: HapAccessory> Serialize for Accessory<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}

impl<T: HapAccessory> HapAccessory for Accessory<T> {
    fn get_id(&self) -> u64 { self.inner.get_id() }

    fn set_id(&mut self, id: u64) { self.inner.set_id(id) }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> { self.inner.get_services() }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> { self.inner.get_mut_services() }

    fn get_mut_information(&mut self) -> &mut AccessoryInformation { self.inner.get_mut_information() }

    fn init_iids(&mut self, accessory_id: u64, event_emitter: pointer::EventEmitter) -> Result<()> {
        self.inner.init_iids(accessory_id, event_emitter)
    }
}

/// The `Information` struct is used to store metadata about an `Accessory` and is converted to the
/// Accessory Information Service of the `Accessory` it is passed to on its creation.
///
/// # Examples
///
/// ```
/// use hap::accessory::{outlet, Information};
///
/// let info = Information {
///     manufacturer: "Acme".into(),
///     model: "A1234".into(),
///     name: "Acme Outlet".into(),
///     serial_number: "11122333".into(),
///     ..Default::default()
/// };
///
/// let outlet = outlet::new(info).unwrap();
/// ```
#[derive(Debug)]
pub struct Information {
    /// Used to cause the `Accessory` to run its identify routine.
    pub identify: bool,
    /// Contains the name of the company whose brand will appear on the `Accessory`, e.g., "Acme".
    pub manufacturer: String,
    /// Contains the manufacturer-specific model of the `Accessory`, e.g. "A1234".
    pub model: String,
    /// Describes the name of the `Accessory`.
    pub name: String,
    /// Contains the manufacturer-specific serial number of the `Accessory`, e.g. "1A2B3C4D5E6F".
    /// The length must be greater than 1.
    pub serial_number: String,
    /// Describes a firmware revision string x[.y[.z]] (e.g. "100.1.1"):
    /// - <x> is the major version number, required.
    /// - <y> is the minor version number, required if it is non-zero or if <z> is present.
    /// - <z> is the revision version number, required if non-zero.
    ///
    /// The firmware revision must follow the below rules:
    /// - <x> is incremented when there is significant change. e.g.,1.0.0, 2.0.0, 3.0.0, etc.
    /// - <y> is incremented when minor changes are introduced such as 1.1.0, 2.1.0, 3.1.0 etc.
    /// - <z> is incremented when bug-fixes are introduced such as 1.0.1, 2.0.1, 3.0.1 etc.
    /// - Subsequent firmware updates can have a lower <y> version only if <x> is incremented
    /// - Subsequent firmware updates can have a lower <z> version only if <x> or <y> is incremented
    ///
    /// The value must change after every firmware update.
    pub firmware_revision: String,
    /// Describes a hardware revision string x[.y[.z]] (e.g. "100.1.1") and tracked when the board
    /// or components of the same accessory is changed:
    /// - <x> is the major version number, required.
    /// - <y> is the minor version number, required if it is non-zero or if <z> is present.
    /// - <z> is the revision version number, required if non-zero.
    ///
    /// The hardware revision must follow the below rules:
    /// - <x> is incremented when there is significant change. e.g.,1.0.0, 2.0.0, 3.0.0, etc.
    /// - <y> is incremented when minor changes are introduced such as 1.1.0, 2.1.0, 3.1.0 etc.
    /// - <z> is incremented when bug-fixes are introduced such as 1.0.1, 2.0.1, 3.0.1 etc.
    /// - Subsequent firmware updates can have a lower <y> version only if <x> is incremented
    /// - Subsequent firmware updates can have a lower <z> version only if <x> or <y> is incremented
    ///
    /// The value must change after every hardware update.
    pub hardware_revision: Option<String>,
    /// When set indicates accessory requires additional setup. Use of Accessory Flags requires
    /// written approval by Apple in advance.
    pub accessory_flags: Option<u32>,
}

impl Information {
    /// Converts the `Information` struct to an Accessory Information Service.
    pub fn to_service(self) -> Result<AccessoryInformation> {
        let mut i = accessory_information::new();
        i.inner.identify.set_value(self.identify)?;
        i.inner.manufacturer.set_value(self.manufacturer)?;
        i.inner.model.set_value(self.model)?;
        i.inner.name.set_value(self.name)?;
        i.inner.serial_number.set_value(self.serial_number)?;
        i.inner.firmware_revision.set_value(self.firmware_revision)?;
        if let Some(v) = self.hardware_revision {
            let mut hr = hardware_revision::new();
            hr.set_value(v)?;
            i.inner.hardware_revision = Some(hr);
        }
        if let Some(v) = self.accessory_flags {
            let mut af = accessory_flags::new();
            af.set_value(v)?;
            i.inner.accessory_flags = Some(af);
        }
        Ok(i)
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
