// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		identify,
		manufacturer,
		model,
		name,
		serial_number,
		firmware_revision,
		hardware_revision,
		accessory_flags,
	},
    HapType,
};

/// Accessory Information Service.
pub type AccessoryInformation = Service<AccessoryInformationInner>;

impl Default for AccessoryInformation {
    fn default() -> AccessoryInformation { new() }
}

/// Inner type of the Accessory Information Service.
#[derive(Default)]
pub struct AccessoryInformationInner {
    /// ID of the Accessory Information Service.
    id: u64,
    /// `HapType` of the Accessory Information Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Identify Characteristic.
	pub identify: identify::Identify,
	/// Manufacturer Characteristic.
	pub manufacturer: manufacturer::Manufacturer,
	/// Model Characteristic.
	pub model: model::Model,
	/// Name Characteristic.
	pub name: name::Name,
	/// Serial Number Characteristic.
	pub serial_number: serial_number::SerialNumber,
	/// Firmware Revision Characteristic.
	pub firmware_revision: firmware_revision::FirmwareRevision,

	/// Hardware Revision Characteristic.
	pub hardware_revision: Option<hardware_revision::HardwareRevision>,
	/// Accessory Flags Characteristic.
	pub accessory_flags: Option<accessory_flags::AccessoryFlags>,
}

impl HapService for AccessoryInformationInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
			&self.identify,
			&self.manufacturer,
			&self.model,
			&self.name,
			&self.serial_number,
			&self.firmware_revision,
		];
		if let Some(c) = &self.hardware_revision {
		    characteristics.push(c);
		}
		if let Some(c) = &self.accessory_flags {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.identify,
			&mut self.manufacturer,
			&mut self.model,
			&mut self.name,
			&mut self.serial_number,
			&mut self.firmware_revision,
		];
		if let Some(c) = &mut self.hardware_revision {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.accessory_flags {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Accessory Information Service.
pub fn new() -> AccessoryInformation {
    AccessoryInformation::new(AccessoryInformationInner {
        hap_type: HapType::AccessoryInformation,
		identify: identify::new(),
		manufacturer: manufacturer::new(),
		model: model::new(),
		name: name::new(),
		serial_number: serial_number::new(),
		firmware_revision: firmware_revision::new(),
		..Default::default()
    })
}
