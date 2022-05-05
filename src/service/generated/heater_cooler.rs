// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		active::ActiveCharacteristic,
		current_heater_cooler_state::CurrentHeaterCoolerStateCharacteristic,
		target_heater_cooler_state::TargetHeaterCoolerStateCharacteristic,
		current_temperature::CurrentTemperatureCharacteristic,
		lock_physical_controls::LockPhysicalControlsCharacteristic,
		name::NameCharacteristic,
		rotation_speed::RotationSpeedCharacteristic,
		swing_mode::SwingModeCharacteristic,
		cooling_threshold_temperature::CoolingThresholdTemperatureCharacteristic,
		heating_threshold_temperature::HeatingThresholdTemperatureCharacteristic,
		temperature_display_units::TemperatureDisplayUnitsCharacteristic,
	},
    HapType,
};

/// Heater-Cooler service.
#[derive(Debug, Default)]
pub struct HeaterCoolerService {
    /// Instance ID of the Heater-Cooler service.
    id: u64,
    /// [`HapType`](HapType) of the Heater-Cooler service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

	/// Active characteristic (required).
	pub active: ActiveCharacteristic,
	/// Current Heater-Cooler State characteristic (required).
	pub current_heater_cooler_state: CurrentHeaterCoolerStateCharacteristic,
	/// Target Heater-Cooler State characteristic (required).
	pub target_heater_cooler_state: TargetHeaterCoolerStateCharacteristic,
	/// Current Temperature characteristic (required).
	pub current_temperature: CurrentTemperatureCharacteristic,
	/// Lock Physical Controls characteristic (optional).
	pub lock_physical_controls: Option<LockPhysicalControlsCharacteristic>,
	/// Name characteristic (optional).
	pub name: Option<NameCharacteristic>,
	/// Rotation Speed characteristic (optional).
	pub rotation_speed: Option<RotationSpeedCharacteristic>,
	/// Swing Mode characteristic (optional).
	pub swing_mode: Option<SwingModeCharacteristic>,
	/// Cooling Threshold Temperature characteristic (optional).
	pub cooling_threshold_temperature: Option<CoolingThresholdTemperatureCharacteristic>,
	/// Heating Threshold Temperature characteristic (optional).
	pub heating_threshold_temperature: Option<HeatingThresholdTemperatureCharacteristic>,
	/// Temperature Display Units characteristic (optional).
	pub temperature_display_units: Option<TemperatureDisplayUnitsCharacteristic>,
}

impl HeaterCoolerService {
    /// Creates a new Heater-Cooler service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::HeaterCooler,
			active: ActiveCharacteristic::new(id  + 1, accessory_id),
			current_heater_cooler_state: CurrentHeaterCoolerStateCharacteristic::new(id + 1  + 1, accessory_id),
			target_heater_cooler_state: TargetHeaterCoolerStateCharacteristic::new(id + 2  + 1, accessory_id),
			current_temperature: CurrentTemperatureCharacteristic::new(id + 3  + 1, accessory_id),
			lock_physical_controls: Some(LockPhysicalControlsCharacteristic::new(id + 1  + 4, accessory_id)),
			name: Some(NameCharacteristic::new(id + 1 + 1  + 4, accessory_id)),
			rotation_speed: Some(RotationSpeedCharacteristic::new(id + 1 + 2  + 4, accessory_id)),
			swing_mode: Some(SwingModeCharacteristic::new(id + 1 + 3  + 4, accessory_id)),
			cooling_threshold_temperature: Some(CoolingThresholdTemperatureCharacteristic::new(id + 1 + 4  + 4, accessory_id)),
			heating_threshold_temperature: Some(HeatingThresholdTemperatureCharacteristic::new(id + 1 + 5  + 4, accessory_id)),
			temperature_display_units: Some(TemperatureDisplayUnitsCharacteristic::new(id + 1 + 6  + 4, accessory_id)),
			..Default::default()
        }
    }
}

impl HapService for HeaterCoolerService {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn set_type(&mut self, hap_type: HapType) {
        self.hap_type = hap_type;
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

    fn get_linked_services(&self) -> Vec<u64> {
        self.linked_services.clone()
    }

    fn set_linked_services(&mut self, linked_services: Vec<u64>) {
        self.linked_services = linked_services;
    }

    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
        for characteristic in self.get_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
        for characteristic in self.get_mut_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
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
		if let Some(c) = &self.rotation_speed {
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
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
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
		if let Some(c) = &mut self.rotation_speed {
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
		characteristics
    }
}

impl Serialize for HeaterCoolerService {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}
