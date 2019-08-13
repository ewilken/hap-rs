// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		active,
		current_heater_cooler_state,
		target_heater_cooler_state,
		current_temperature,
		lock_physical_controls,
		name,
		swing_mode,
		cooling_threshold_temperature,
		heating_threshold_temperature,
		temperature_display_units,
		rotation_speed,
	},
    HapType,
};

/// Heater Cooler Service.
pub type HeaterCooler = Service<HeaterCoolerInner>;

impl Default for HeaterCooler {
    fn default() -> HeaterCooler { new() }
}

/// Inner type of the Heater Cooler Service.
#[derive(Default)]
pub struct HeaterCoolerInner {
    /// ID of the Heater Cooler Service.
    id: u64,
    /// `HapType` of the Heater Cooler Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Active Characteristic.
	pub active: active::Active,
	/// Current Heater Cooler State Characteristic.
	pub current_heater_cooler_state: current_heater_cooler_state::CurrentHeaterCoolerState,
	/// Target Heater Cooler State Characteristic.
	pub target_heater_cooler_state: target_heater_cooler_state::TargetHeaterCoolerState,
	/// Current Temperature Characteristic.
	pub current_temperature: current_temperature::CurrentTemperature,

	/// Lock Physical Controls Characteristic.
	pub lock_physical_controls: Option<lock_physical_controls::LockPhysicalControls>,
	/// Name Characteristic.
	pub name: Option<name::Name>,
	/// Swing Mode Characteristic.
	pub swing_mode: Option<swing_mode::SwingMode>,
	/// Cooling Threshold Temperature Characteristic.
	pub cooling_threshold_temperature: Option<cooling_threshold_temperature::CoolingThresholdTemperature>,
	/// Heating Threshold Temperature Characteristic.
	pub heating_threshold_temperature: Option<heating_threshold_temperature::HeatingThresholdTemperature>,
	/// Temperature Display Units Characteristic.
	pub temperature_display_units: Option<temperature_display_units::TemperatureDisplayUnits>,
	/// Rotation Speed Characteristic.
	pub rotation_speed: Option<rotation_speed::RotationSpeed>,
}

impl HapService for HeaterCoolerInner {
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
			&self.active,
			&self.current_heater_cooler_state,
			&self.target_heater_cooler_state,
			&self.current_temperature,
		];
		if let Some(c) = &self.lock_physical_controls {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &self.swing_mode {
		    characteristics.push(c);
		}
		if let Some(c) = &self.cooling_threshold_temperature {
		    characteristics.push(c);
		}
		if let Some(c) = &self.heating_threshold_temperature {
		    characteristics.push(c);
		}
		if let Some(c) = &self.temperature_display_units {
		    characteristics.push(c);
		}
		if let Some(c) = &self.rotation_speed {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> {
        let mut characteristics: Vec<&mut HapCharacteristic> = vec![
			&mut self.active,
			&mut self.current_heater_cooler_state,
			&mut self.target_heater_cooler_state,
			&mut self.current_temperature,
		];
		if let Some(c) = &mut self.lock_physical_controls {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.swing_mode {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.cooling_threshold_temperature {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.heating_threshold_temperature {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.temperature_display_units {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.rotation_speed {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Heater Cooler Service.
pub fn new() -> HeaterCooler {
    HeaterCooler::new(HeaterCoolerInner {
        hap_type: HapType::HeaterCooler,
		active: active::new(),
		current_heater_cooler_state: current_heater_cooler_state::new(),
		target_heater_cooler_state: target_heater_cooler_state::new(),
		current_temperature: current_temperature::new(),
		..Default::default()
    })
}
