// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		motion_detected,
		status_active,
		status_fault,
		status_tampered,
		status_low_battery,
		name,
	},
    HapType,
};

/// Motion Sensor Service.
pub type MotionSensor = Service<MotionSensorInner>;

impl Default for MotionSensor {
    fn default() -> MotionSensor { new() }
}

/// Inner type of the Motion Sensor Service.
#[derive(Default)]
pub struct MotionSensorInner {
    /// ID of the Motion Sensor Service.
    id: u64,
    /// `HapType` of the Motion Sensor Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Motion Detected Characteristic.
	pub motion_detected: motion_detected::MotionDetected,

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

impl HapService for MotionSensorInner {
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

    fn get_characteristics(&self) -> Vec<&HapCharacteristic> {
        let mut characteristics: Vec<&HapCharacteristic> = vec![
			&self.motion_detected,
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

    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> {
        let mut characteristics: Vec<&mut HapCharacteristic> = vec![
			&mut self.motion_detected,
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

/// Creates a new Motion Sensor Service.
pub fn new() -> MotionSensor {
    MotionSensor::new(MotionSensorInner {
        hap_type: HapType::MotionSensor,
		motion_detected: motion_detected::new(),
		..Default::default()
    })
}
