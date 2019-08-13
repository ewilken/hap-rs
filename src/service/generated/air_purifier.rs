// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		active,
		current_air_purifier_state,
		target_air_purifier_state,
		lock_physical_controls,
		name,
		swing_mode,
		rotation_speed,
	},
    HapType,
};

/// Air Purifier Service.
pub type AirPurifier = Service<AirPurifierInner>;

impl Default for AirPurifier {
    fn default() -> AirPurifier { new() }
}

/// Inner type of the Air Purifier Service.
#[derive(Default)]
pub struct AirPurifierInner {
    /// ID of the Air Purifier Service.
    id: u64,
    /// `HapType` of the Air Purifier Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Active Characteristic.
	pub active: active::Active,
	/// Current Air Purifier State Characteristic.
	pub current_air_purifier_state: current_air_purifier_state::CurrentAirPurifierState,
	/// Target Air Purifier State Characteristic.
	pub target_air_purifier_state: target_air_purifier_state::TargetAirPurifierState,

	/// Lock Physical Controls Characteristic.
	pub lock_physical_controls: Option<lock_physical_controls::LockPhysicalControls>,
	/// Name Characteristic.
	pub name: Option<name::Name>,
	/// Swing Mode Characteristic.
	pub swing_mode: Option<swing_mode::SwingMode>,
	/// Rotation Speed Characteristic.
	pub rotation_speed: Option<rotation_speed::RotationSpeed>,
}

impl HapService for AirPurifierInner {
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
			&self.current_air_purifier_state,
			&self.target_air_purifier_state,
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
		if let Some(c) = &self.rotation_speed {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> {
        let mut characteristics: Vec<&mut HapCharacteristic> = vec![
			&mut self.active,
			&mut self.current_air_purifier_state,
			&mut self.target_air_purifier_state,
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
		if let Some(c) = &mut self.rotation_speed {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Air Purifier Service.
pub fn new() -> AirPurifier {
    AirPurifier::new(AirPurifierInner {
        hap_type: HapType::AirPurifier,
		active: active::new(),
		current_air_purifier_state: current_air_purifier_state::new(),
		target_air_purifier_state: target_air_purifier_state::new(),
		..Default::default()
    })
}
