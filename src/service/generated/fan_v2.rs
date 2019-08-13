// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		active,
		current_fan_state,
		target_fan_state,
		lock_physical_controls,
		name,
		rotation_direction,
		rotation_speed,
		swing_mode,
	},
    HapType,
};

/// Fan v2 Service.
pub type Fanv2 = Service<Fanv2Inner>;

impl Default for Fanv2 {
    fn default() -> Fanv2 { new() }
}

/// Inner type of the Fan v2 Service.
#[derive(Default)]
pub struct Fanv2Inner {
    /// ID of the Fan v2 Service.
    id: u64,
    /// `HapType` of the Fan v2 Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Active Characteristic.
	pub active: active::Active,

	/// Current Fan State Characteristic.
	pub current_fan_state: Option<current_fan_state::CurrentFanState>,
	/// Target Fan State Characteristic.
	pub target_fan_state: Option<target_fan_state::TargetFanState>,
	/// Lock Physical Controls Characteristic.
	pub lock_physical_controls: Option<lock_physical_controls::LockPhysicalControls>,
	/// Name Characteristic.
	pub name: Option<name::Name>,
	/// Rotation Direction Characteristic.
	pub rotation_direction: Option<rotation_direction::RotationDirection>,
	/// Rotation Speed Characteristic.
	pub rotation_speed: Option<rotation_speed::RotationSpeed>,
	/// Swing Mode Characteristic.
	pub swing_mode: Option<swing_mode::SwingMode>,
}

impl HapService for Fanv2Inner {
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
			&self.active,
		];
		if let Some(c) = &self.current_fan_state {
		    characteristics.push(c);
		}
		if let Some(c) = &self.target_fan_state {
		    characteristics.push(c);
		}
		if let Some(c) = &self.lock_physical_controls {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &self.rotation_direction {
		    characteristics.push(c);
		}
		if let Some(c) = &self.rotation_speed {
		    characteristics.push(c);
		}
		if let Some(c) = &self.swing_mode {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.active,
		];
		if let Some(c) = &mut self.current_fan_state {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.target_fan_state {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.lock_physical_controls {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.rotation_direction {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.rotation_speed {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.swing_mode {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Fan v2 Service.
pub fn new() -> Fanv2 {
    Fanv2::new(Fanv2Inner {
        hap_type: HapType::Fanv2,
		active: active::new(),
		..Default::default()
    })
}
