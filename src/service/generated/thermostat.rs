// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		current_heating_cooling_state,
		target_heating_cooling_state,
		current_temperature,
		target_temperature,
		temperature_display_units,
		current_relative_humidity,
		target_relative_humidity,
		cooling_threshold_temperature,
		heating_threshold_temperature,
		name,
	},
    HapType,
};

/// Thermostat Service.
pub type Thermostat = Service<ThermostatInner>;

impl Default for Thermostat {
    fn default() -> Thermostat { new() }
}

/// Inner type of the Thermostat Service.
#[derive(Default)]
pub struct ThermostatInner {
    /// ID of the Thermostat Service.
    id: u64,
    /// `HapType` of the Thermostat Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Current Heating Cooling State Characteristic.
	pub current_heating_cooling_state: current_heating_cooling_state::CurrentHeatingCoolingState,
	/// Target Heating Cooling State Characteristic.
	pub target_heating_cooling_state: target_heating_cooling_state::TargetHeatingCoolingState,
	/// Current Temperature Characteristic.
	pub current_temperature: current_temperature::CurrentTemperature,
	/// Target Temperature Characteristic.
	pub target_temperature: target_temperature::TargetTemperature,
	/// Temperature Display Units Characteristic.
	pub temperature_display_units: temperature_display_units::TemperatureDisplayUnits,

	/// Current Relative Humidity Characteristic.
	pub current_relative_humidity: Option<current_relative_humidity::CurrentRelativeHumidity>,
	/// Target Relative Humidity Characteristic.
	pub target_relative_humidity: Option<target_relative_humidity::TargetRelativeHumidity>,
	/// Cooling Threshold Temperature Characteristic.
	pub cooling_threshold_temperature: Option<cooling_threshold_temperature::CoolingThresholdTemperature>,
	/// Heating Threshold Temperature Characteristic.
	pub heating_threshold_temperature: Option<heating_threshold_temperature::HeatingThresholdTemperature>,
	/// Name Characteristic.
	pub name: Option<name::Name>,
}

impl HapService for ThermostatInner {
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
			&self.current_heating_cooling_state,
			&self.target_heating_cooling_state,
			&self.current_temperature,
			&self.target_temperature,
			&self.temperature_display_units,
		];
		if let Some(c) = &self.current_relative_humidity {
		    characteristics.push(c);
		}
		if let Some(c) = &self.target_relative_humidity {
		    characteristics.push(c);
		}
		if let Some(c) = &self.cooling_threshold_temperature {
		    characteristics.push(c);
		}
		if let Some(c) = &self.heating_threshold_temperature {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.current_heating_cooling_state,
			&mut self.target_heating_cooling_state,
			&mut self.current_temperature,
			&mut self.target_temperature,
			&mut self.temperature_display_units,
		];
		if let Some(c) = &mut self.current_relative_humidity {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.target_relative_humidity {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.cooling_threshold_temperature {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.heating_threshold_temperature {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Thermostat Service.
pub fn new() -> Thermostat {
    Thermostat::new(ThermostatInner {
        hap_type: HapType::Thermostat,
		current_heating_cooling_state: current_heating_cooling_state::new(),
		target_heating_cooling_state: target_heating_cooling_state::new(),
		current_temperature: current_temperature::new(),
		target_temperature: target_temperature::new(),
		temperature_display_units: temperature_display_units::new(),
		..Default::default()
    })
}
