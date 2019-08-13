// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		current_ambient_light_level,
		name,
		status_active,
		status_fault,
		status_tampered,
		status_low_battery,
	},
    HapType,
};

/// Light Sensor Service.
pub type LightSensor = Service<LightSensorInner>;

impl Default for LightSensor {
    fn default() -> LightSensor { new() }
}

/// Inner type of the Light Sensor Service.
#[derive(Default)]
pub struct LightSensorInner {
    /// ID of the Light Sensor Service.
    id: u64,
    /// `HapType` of the Light Sensor Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Current Ambient Light Level Characteristic.
	pub current_ambient_light_level: current_ambient_light_level::CurrentAmbientLightLevel,

	/// Name Characteristic.
	pub name: Option<name::Name>,
	/// Status Active Characteristic.
	pub status_active: Option<status_active::StatusActive>,
	/// Status Fault Characteristic.
	pub status_fault: Option<status_fault::StatusFault>,
	/// Status Tampered Characteristic.
	pub status_tampered: Option<status_tampered::StatusTampered>,
	/// Status Low Battery Characteristic.
	pub status_low_battery: Option<status_low_battery::StatusLowBattery>,
}

impl HapService for LightSensorInner {
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
			&self.current_ambient_light_level,
		];
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_active {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_fault {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_tampered {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_low_battery {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.current_ambient_light_level,
		];
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_active {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_fault {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_tampered {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_low_battery {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Light Sensor Service.
pub fn new() -> LightSensor {
    LightSensor::new(LightSensorInner {
        hap_type: HapType::LightSensor,
		current_ambient_light_level: current_ambient_light_level::new(),
		..Default::default()
    })
}
