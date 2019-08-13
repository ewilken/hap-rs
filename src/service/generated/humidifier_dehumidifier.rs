// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		current_relative_humidity,
		current_humidifier_dehumidifier_state,
		target_humidifier_dehumidifier_state,
		active,
		lock_physical_controls,
		name,
		swing_mode,
		water_level,
		relative_humidity_dehumidifier_threshold,
		relative_humidity_humidifier_threshold,
		rotation_speed,
	},
    HapType,
};

/// Humidifier Dehumidifier Service.
pub type HumidifierDehumidifier = Service<HumidifierDehumidifierInner>;

impl Default for HumidifierDehumidifier {
    fn default() -> HumidifierDehumidifier { new() }
}

/// Inner type of the Humidifier Dehumidifier Service.
#[derive(Default)]
pub struct HumidifierDehumidifierInner {
    /// ID of the Humidifier Dehumidifier Service.
    id: u64,
    /// `HapType` of the Humidifier Dehumidifier Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Current Relative Humidity Characteristic.
	pub current_relative_humidity: current_relative_humidity::CurrentRelativeHumidity,
	/// Current Humidifier Dehumidifier State Characteristic.
	pub current_humidifier_dehumidifier_state: current_humidifier_dehumidifier_state::CurrentHumidifierDehumidifierState,
	/// Target Humidifier Dehumidifier State Characteristic.
	pub target_humidifier_dehumidifier_state: target_humidifier_dehumidifier_state::TargetHumidifierDehumidifierState,
	/// Active Characteristic.
	pub active: active::Active,

	/// Lock Physical Controls Characteristic.
	pub lock_physical_controls: Option<lock_physical_controls::LockPhysicalControls>,
	/// Name Characteristic.
	pub name: Option<name::Name>,
	/// Swing Mode Characteristic.
	pub swing_mode: Option<swing_mode::SwingMode>,
	/// Water Level Characteristic.
	pub water_level: Option<water_level::WaterLevel>,
	/// Relative Humidity Dehumidifier Threshold Characteristic.
	pub relative_humidity_dehumidifier_threshold: Option<relative_humidity_dehumidifier_threshold::RelativeHumidityDehumidifierThreshold>,
	/// Relative Humidity Humidifier Threshold Characteristic.
	pub relative_humidity_humidifier_threshold: Option<relative_humidity_humidifier_threshold::RelativeHumidityHumidifierThreshold>,
	/// Rotation Speed Characteristic.
	pub rotation_speed: Option<rotation_speed::RotationSpeed>,
}

impl HapService for HumidifierDehumidifierInner {
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
			&self.current_relative_humidity,
			&self.current_humidifier_dehumidifier_state,
			&self.target_humidifier_dehumidifier_state,
			&self.active,
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
		if let Some(c) = &self.water_level {
		    characteristics.push(c);
		}
		if let Some(c) = &self.relative_humidity_dehumidifier_threshold {
		    characteristics.push(c);
		}
		if let Some(c) = &self.relative_humidity_humidifier_threshold {
		    characteristics.push(c);
		}
		if let Some(c) = &self.rotation_speed {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> {
        let mut characteristics: Vec<&mut HapCharacteristic> = vec![
			&mut self.current_relative_humidity,
			&mut self.current_humidifier_dehumidifier_state,
			&mut self.target_humidifier_dehumidifier_state,
			&mut self.active,
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
		if let Some(c) = &mut self.water_level {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.relative_humidity_dehumidifier_threshold {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.relative_humidity_humidifier_threshold {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.rotation_speed {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Humidifier Dehumidifier Service.
pub fn new() -> HumidifierDehumidifier {
    HumidifierDehumidifier::new(HumidifierDehumidifierInner {
        hap_type: HapType::HumidifierDehumidifier,
		current_relative_humidity: current_relative_humidity::new(),
		current_humidifier_dehumidifier_state: current_humidifier_dehumidifier_state::new(),
		target_humidifier_dehumidifier_state: target_humidifier_dehumidifier_state::new(),
		active: active::new(),
		..Default::default()
    })
}
