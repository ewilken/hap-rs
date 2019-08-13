// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		leak_detected,
		status_active,
		status_fault,
		status_tampered,
		status_low_battery,
		name,
	},
    HapType,
};

/// Leak Sensor Service.
pub type LeakSensor = Service<LeakSensorInner>;

impl Default for LeakSensor {
    fn default() -> LeakSensor { new() }
}

/// Inner type of the Leak Sensor Service.
#[derive(Default)]
pub struct LeakSensorInner {
    /// ID of the Leak Sensor Service.
    id: u64,
    /// `HapType` of the Leak Sensor Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Leak Detected Characteristic.
	pub leak_detected: leak_detected::LeakDetected,

	/// Status Active Characteristic.
	pub status_active: Option<status_active::StatusActive>,
	/// Status Fault Characteristic.
	pub status_fault: Option<status_fault::StatusFault>,
	/// Status Tampered Characteristic.
	pub status_tampered: Option<status_tampered::StatusTampered>,
	/// Status Low Battery Characteristic.
	pub status_low_battery: Option<status_low_battery::StatusLowBattery>,
	/// Name Characteristic.
	pub name: Option<name::Name>,
}

impl HapService for LeakSensorInner {
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
			&self.leak_detected,
		];
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
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.leak_detected,
		];
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
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Leak Sensor Service.
pub fn new() -> LeakSensor {
    LeakSensor::new(LeakSensorInner {
        hap_type: HapType::LeakSensor,
		leak_detected: leak_detected::new(),
		..Default::default()
    })
}
