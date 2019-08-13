// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		carbon_monoxide_detected,
		status_active,
		status_fault,
		status_low_battery,
		status_tampered,
		carbon_monoxide_level,
		carbon_monoxide_peak_level,
		name,
	},
    HapType,
};

/// Carbon Monoxide Sensor Service.
pub type CarbonMonoxideSensor = Service<CarbonMonoxideSensorInner>;

impl Default for CarbonMonoxideSensor {
    fn default() -> CarbonMonoxideSensor { new() }
}

/// Inner type of the Carbon Monoxide Sensor Service.
#[derive(Default)]
pub struct CarbonMonoxideSensorInner {
    /// ID of the Carbon Monoxide Sensor Service.
    id: u64,
    /// `HapType` of the Carbon Monoxide Sensor Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Carbon Monoxide Detected Characteristic.
	pub carbon_monoxide_detected: carbon_monoxide_detected::CarbonMonoxideDetected,

	/// Status Active Characteristic.
	pub status_active: Option<status_active::StatusActive>,
	/// Status Fault Characteristic.
	pub status_fault: Option<status_fault::StatusFault>,
	/// Status Low Battery Characteristic.
	pub status_low_battery: Option<status_low_battery::StatusLowBattery>,
	/// Status Tampered Characteristic.
	pub status_tampered: Option<status_tampered::StatusTampered>,
	/// Carbon Monoxide Level Characteristic.
	pub carbon_monoxide_level: Option<carbon_monoxide_level::CarbonMonoxideLevel>,
	/// Carbon Monoxide Peak Level Characteristic.
	pub carbon_monoxide_peak_level: Option<carbon_monoxide_peak_level::CarbonMonoxidePeakLevel>,
	/// Name Characteristic.
	pub name: Option<name::Name>,
}

impl HapService for CarbonMonoxideSensorInner {
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
			&self.carbon_monoxide_detected,
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
		if let Some(c) = &self.carbon_monoxide_level {
		    characteristics.push(c);
		}
		if let Some(c) = &self.carbon_monoxide_peak_level {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> {
        let mut characteristics: Vec<&mut HapCharacteristic> = vec![
			&mut self.carbon_monoxide_detected,
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
		if let Some(c) = &mut self.carbon_monoxide_level {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.carbon_monoxide_peak_level {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Carbon Monoxide Sensor Service.
pub fn new() -> CarbonMonoxideSensor {
    CarbonMonoxideSensor::new(CarbonMonoxideSensorInner {
        hap_type: HapType::CarbonMonoxideSensor,
		carbon_monoxide_detected: carbon_monoxide_detected::new(),
		..Default::default()
    })
}
