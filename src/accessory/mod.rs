use erased_serde::serialize_trait_object;
use futures::executor;

use crate::{
    characteristic::{
        accessory_flags::AccessoryFlagsCharacteristic,
        application_matching_identifier::ApplicationMatchingIdentifierCharacteristic,
        configured_name::ConfiguredNameCharacteristic,
        firmware_revision::FirmwareRevisionCharacteristic,
        hardware_finish::HardwareFinishCharacteristic,
        hardware_revision::HardwareRevisionCharacteristic,
        product_data::ProductDataCharacteristic,
        software_revision::SoftwareRevisionCharacteristic,
        HapCharacteristic,
    },
    pointer,
    service::{accessory_information::AccessoryInformationService, HapService},
    HapType,
    Result,
};

mod category;
mod defined;
mod generated;

pub use crate::accessory::{category::AccessoryCategory, defined::*, generated::*};

/// [`HapAccessory`](HapAccessory) is implemented by every HAP accessory.
pub trait HapAccessory: HapAccessorySetup + erased_serde::Serialize + Send + Sync {
    /// Returns the ID of the accessory.
    fn get_id(&self) -> u64;
    /// Sets the ID of the accessory.
    fn set_id(&mut self, id: u64);
    /// Returns a reference to a specific service of the accessory if it's present on it.
    fn get_service(&self, hap_type: HapType) -> Option<&dyn HapService>;
    /// Returns a mutable reference to a specific service of the accessory if it's present on it.
    fn get_mut_service(&mut self, hap_type: HapType) -> Option<&mut dyn HapService>;
    /// Returns references to all services of the accessory.
    fn get_services(&self) -> Vec<&dyn HapService>;
    /// Returns mutable references to all services of the accessory.
    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService>;
}

serialize_trait_object!(HapAccessory);

/// [`HapAccessorySetup`](HapAccessorySetup) is implemented by every HAP accessory to provide helper methods used by the
/// HAP server for setup purposes. It's not meant to be used by a consumer of the library.
pub trait HapAccessorySetup {
    /// Sets a pointer to an `EventEmitter` on all characteristics of the accessory.
    fn set_event_emitter_on_characteristics(&mut self, event_emitter: Option<pointer::EventEmitter>);
}

impl<H> HapAccessorySetup for H
where
    H: HapAccessory,
{
    fn set_event_emitter_on_characteristics(&mut self, event_emitter: Option<pointer::EventEmitter>) {
        for service in self.get_mut_services() {
            for characteristic in service.get_mut_characteristics() {
                characteristic.set_event_emitter(event_emitter.clone());
            }
        }
    }
}

/// The [`AccessoryInformation`](AccessoryInformation) struct is used to conveniently store metadata about an accessory
/// and is converted to the [`AccessoryInformationService`](AccessoryInformationService) of the accessory it is passed
/// to on its creation.
///
/// # Examples
///
/// ```
/// use hap::accessory::{outlet::OutletAccessory, AccessoryInformation};
///
/// let information = AccessoryInformation {
///     manufacturer: "Acme".into(),
///     model: "A1234".into(),
///     name: "Acme Outlet".into(),
///     serial_number: "1A2B3C4D5E6F".into(),
///     ..Default::default()
/// };
///
/// let outlet = OutletAccessory::new(1, information).unwrap();
/// ```
#[derive(Debug)]
pub struct AccessoryInformation {
    // TODO - include all possible fields of AccessoryInformationService
    /// Contains the name of the company whose brand will appear on the accessory, e.g., "Acme".
    pub manufacturer: String,
    /// Contains the manufacturer-specific model of the accessory, e.g. "A1234".
    pub model: String,
    /// Describes the name of the accessory.
    pub name: String,
    /// Contains the manufacturer-specific serial number of the accessory, e.g. "1A2B3C4D5E6F".
    /// The length must be greater than 1.
    pub serial_number: String,
    /// When set indicates accessory requires additional setup. Use of accessory flags requires
    /// written approval by Apple in advance.
    pub accessory_flags: Option<u32>,
    pub application_matching_identifier: Option<Vec<u8>>,
    pub configured_name: Option<String>,
    /// Describes a firmware revision string x[.y[.z]] (e.g. "100.1.1"):
    /// - <x> is the major version number, required.
    /// - <y> is the minor version number, required if it is non-zero or if <z> is present.
    /// - <z> is the revision version number, required if non-zero.
    ///
    /// The firmware revision must follow the below rules:
    /// - <x> is incremented when there is significant change. e.g., 1.0.0, 2.0.0, 3.0.0, etc.
    /// - <y> is incremented when minor changes are introduced such as 1.1.0, 2.1.0, 3.1.0 etc.
    /// - <z> is incremented when bug-fixes are introduced such as 1.0.1, 2.0.1, 3.0.1 etc.
    /// - Subsequent firmware updates can have a lower <y> version only if <x> is incremented
    /// - Subsequent firmware updates can have a lower <z> version only if <x> or <y> is incremented
    ///
    /// The value must change after every firmware update.
    pub firmware_revision: Option<String>,
    pub hardware_finish: Option<Vec<u8>>,
    /// Describes a hardware revision string x[.y[.z]] (e.g. "100.1.1") and tracked when the board
    /// or components of the same accessory is changed:
    /// - <x> is the major version number, required.
    /// - <y> is the minor version number, required if it is non-zero or if <z> is present.
    /// - <z> is the revision version number, required if non-zero.
    ///
    /// The hardware revision must follow the below rules:
    /// - <x> is incremented when there is significant change. e.g., 1.0.0, 2.0.0, 3.0.0, etc.
    /// - <y> is incremented when minor changes are introduced such as 1.1.0, 2.1.0, 3.1.0 etc.
    /// - <z> is incremented when bug-fixes are introduced such as 1.0.1, 2.0.1, 3.0.1 etc.
    /// - Subsequent firmware updates can have a lower <y> version only if <x> is incremented
    /// - Subsequent firmware updates can have a lower <z> version only if <x> or <y> is incremented
    ///
    /// The value must change after every hardware update.
    pub hardware_revision: Option<String>,
    pub product_data: Option<Vec<u8>>,
    pub software_revision: Option<String>,
}

impl AccessoryInformation {
    /// Converts the `Information` struct to an Accessory Information Service.
    pub fn to_service(self, id: u64, accessory_id: u64) -> Result<AccessoryInformationService> {
        let mut i = AccessoryInformationService::new(id, accessory_id);

        executor::block_on(i.identify.set_value(serde_json::Value::Bool(false)))?;
        executor::block_on(i.manufacturer.set_value(serde_json::Value::String(self.manufacturer)))?;
        executor::block_on(i.model.set_value(serde_json::Value::String(self.model)))?;
        executor::block_on(i.name.set_value(serde_json::Value::String(self.name)))?;
        executor::block_on(i.serial_number.set_value(serde_json::Value::String(self.serial_number)))?;

        if let Some(v) = self.accessory_flags {
            let mut c = AccessoryFlagsCharacteristic::new(id + 6, accessory_id);
            executor::block_on(c.set_value(v.into()))?;
            i.accessory_flags = Some(c);
        } else {
            i.accessory_flags = None;
        }

        if let Some(v) = self.application_matching_identifier {
            let mut c = ApplicationMatchingIdentifierCharacteristic::new(id + 7, accessory_id);
            executor::block_on(c.set_value(v.into()))?;
            i.application_matching_identifier = Some(c);
        } else {
            i.application_matching_identifier = None;
        }

        if let Some(v) = self.configured_name {
            let mut c = ConfiguredNameCharacteristic::new(id + 8, accessory_id);
            executor::block_on(c.set_value(v.into()))?;
            i.configured_name = Some(c);
        } else {
            i.configured_name = None;
        }

        if let Some(v) = self.firmware_revision {
            let mut c = FirmwareRevisionCharacteristic::new(id + 9, accessory_id);
            executor::block_on(c.set_value(v.into()))?;
            i.firmware_revision = Some(c);
        } else {
            i.firmware_revision = None;
        }

        if let Some(v) = self.hardware_finish {
            let mut c = HardwareFinishCharacteristic::new(id + 12, accessory_id);
            executor::block_on(c.set_value(v.into()))?;
            i.hardware_finish = Some(c);
        } else {
            i.hardware_finish = None;
        }

        if let Some(v) = self.hardware_revision {
            let mut c = HardwareRevisionCharacteristic::new(id + 10, accessory_id);
            executor::block_on(c.set_value(v.into()))?;
            i.hardware_revision = Some(c);
        } else {
            i.hardware_revision = None;
        }

        if let Some(v) = self.product_data {
            let mut c = ProductDataCharacteristic::new(id + 12, accessory_id);
            executor::block_on(c.set_value(v.into()))?;
            i.product_data = Some(c);
        } else {
            i.product_data = None;
        }

        if let Some(v) = self.software_revision {
            let mut c = SoftwareRevisionCharacteristic::new(id + 11, accessory_id);
            executor::block_on(c.set_value(v.into()))?;
            i.software_revision = Some(c);
        } else {
            i.software_revision = None;
        }

        Ok(i)
    }
}

impl Default for AccessoryInformation {
    fn default() -> Self {
        Self {
            manufacturer: "undefined".into(),
            model: "undefined".into(),
            name: "undefined".into(),
            serial_number: "undefined".into(),
            accessory_flags: None,
            application_matching_identifier: None,
            configured_name: None,
            firmware_revision: None,
            hardware_finish: None,
            hardware_revision: None,
            product_data: None,
            software_revision: None,
        }
    }
}
