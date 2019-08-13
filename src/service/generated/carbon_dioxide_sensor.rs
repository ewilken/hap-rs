// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		carbon_dioxide_detected,
		status_active,
		status_fault,
		status_low_battery,
		status_tampered,
		carbon_dioxide_level,
		carbon_dioxide_peak_level,
		name,
	},
    HapType,
};

/// Carbon Dioxide Sensor Service.
pub type CarbonDioxideSensor = Service<CarbonDioxideSensorInner>;

impl Default for CarbonDioxideSensor {
    fn default() -> CarbonDioxideSensor { new() }
}

/// Inner type of the Carbon Dioxide Sensor Service.
#[derive(Default)]
pub struct CarbonDioxideSensorInner {
    /// ID of the Carbon Dioxide Sensor Service.
    id: u64,
    /// `HapType` of the Carbon Dioxide Sensor Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Carbon Dioxide Detected Characteristic.
	pub carbon_dioxide_detected: carbon_dioxide_detected::CarbonDioxideDetected,

	/// Status Active Characteristic.
	pub status_active: Option<status_active::StatusActive>,
	/// Status Fault Characteristic.
	pub status_fault: Option<status_fault::StatusFault>,
	/// Status Low Battery Characteristic.
	pub status_low_battery: Option<status_low_battery::StatusLowBattery>,
	/// Status Tampered Characteristic.
	pub status_tampered: Option<status_tampered::StatusTampered>,
	/// Carbon Dioxide Level Characteristic.
	pub carbon_dioxide_level: Option<carbon_dioxide_level::CarbonDioxideLevel>,
	/// Carbon Dioxide Peak Level Characteristic.
	pub carbon_dioxide_peak_level: Option<carbon_dioxide_peak_level::CarbonDioxidePeakLevel>,
	/// Name Characteristic.
	pub name: Option<name::Name>,
}

impl HapService for CarbonDioxideSensorInner {
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
			&self.carbon_dioxide_detected,
		];
		if let Some(c) = &self.status_active {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_fault {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_low_battery {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_tampered {
		    characteristics.push(c);
		}
		if let Some(c) = &self.carbon_dioxide_level {
		    characteristics.push(c);
		}
		if let Some(c) = &self.carbon_dioxide_peak_level {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.carbon_dioxide_detected,
		];
		if let Some(c) = &mut self.status_active {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_fault {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_low_battery {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_tampered {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.carbon_dioxide_level {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.carbon_dioxide_peak_level {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Carbon Dioxide Sensor Service.
pub fn new() -> CarbonDioxideSensor {
    CarbonDioxideSensor::new(CarbonDioxideSensorInner {
        hap_type: HapType::CarbonDioxideSensor,
		carbon_dioxide_detected: carbon_dioxide_detected::new(),
		..Default::default()
    })
}
