use erased_serde::serialize_trait_object;
use futures::executor;

use crate::{
    characteristic::{
        accessory_flags::AccessoryFlagsCharacteristic,
        hardware_revision::HardwareRevisionCharacteristic,
        HapCharacteristic,
    },
    service::{accessory_information::AccessoryInformationService, HapService},
    Result,
};

mod category;
// mod defined;
mod generated;

pub use crate::accessory::{category::Category, generated::*};

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
    fn get_services(&self) -> Vec<&(dyn HapAccessoryService + Send + Sync)>;
    /// Returns mutable references to the Services of an Accessory.
    fn get_mut_services(&mut self) -> Vec<&mut (dyn HapAccessoryService + Send + Sync)>;
    /// Returns a mutable reference to the Accessory Information Service of an Accessory.
    fn get_mut_information(&mut self) -> &mut AccessoryInformationService;
}

/// The `Information` struct is used to store metadata about an `Accessory` and is converted to the Accessory
/// Information Service of the `Accessory` it is passed to on its creation.
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
    pub fn to_service(self, accessory_id: u64) -> Result<AccessoryInformationService> {
        let mut i = AccessoryInformationService::new(1, accessory_id);
        executor::block_on(i.identify.set_value(serde_json::Value::Bool(self.identify)))?;
        executor::block_on(i.manufacturer.set_value(serde_json::Value::String(self.manufacturer)))?;
        executor::block_on(i.model.set_value(serde_json::Value::String(self.model)))?;
        executor::block_on(i.name.set_value(serde_json::Value::String(self.name)))?;
        executor::block_on(i.serial_number.set_value(serde_json::Value::String(self.serial_number)))?;
        executor::block_on(
            i.firmware_revision
                .set_value(serde_json::Value::String(self.firmware_revision)),
        )?;
        if let Some(v) = self.hardware_revision {
            let mut hr = HardwareRevisionCharacteristic::new(7, accessory_id);
            executor::block_on(hr.set_value(serde_json::Value::String(v)))?;
            i.hardware_revision = Some(hr);
        }
        if let Some(v) = self.accessory_flags {
            let mut af = AccessoryFlagsCharacteristic::new(8, accessory_id);
            executor::block_on(af.set_value(serde_json::Value::Number(v.into())))?;
            i.accessory_flags = Some(af);
        }
        Ok(i)
    }
}

impl Default for Information {
    fn default() -> Self {
        Self {
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
