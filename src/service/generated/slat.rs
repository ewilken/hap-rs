// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		slat_type,
		current_slat_state,
		name,
		current_tilt_angle,
		target_tilt_angle,
		swing_mode,
	},
    HapType,
};

/// Slat Service.
pub type Slat = Service<SlatInner>;

impl Default for Slat {
    fn default() -> Slat { new() }
}

/// Inner type of the Slat Service.
#[derive(Default)]
pub struct SlatInner {
    /// ID of the Slat Service.
    id: u64,
    /// `HapType` of the Slat Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Slat Type Characteristic.
	pub slat_type: slat_type::SlatType,
	/// Current Slat State Characteristic.
	pub current_slat_state: current_slat_state::CurrentSlatState,

	/// Name Characteristic.
	pub name: Option<name::Name>,
	/// Current Tilt Angle Characteristic.
	pub current_tilt_angle: Option<current_tilt_angle::CurrentTiltAngle>,
	/// Target Tilt Angle Characteristic.
	pub target_tilt_angle: Option<target_tilt_angle::TargetTiltAngle>,
	/// Swing Mode Characteristic.
	pub swing_mode: Option<swing_mode::SwingMode>,
}

impl HapService for SlatInner {
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
			&self.slat_type,
			&self.current_slat_state,
		];
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &self.current_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &self.target_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &self.swing_mode {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> {
        let mut characteristics: Vec<&mut HapCharacteristic> = vec![
			&mut self.slat_type,
			&mut self.current_slat_state,
		];
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.current_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.target_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.swing_mode {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Slat Service.
pub fn new() -> Slat {
    Slat::new(SlatInner {
        hap_type: HapType::Slat,
		slat_type: slat_type::new(),
		current_slat_state: current_slat_state::new(),
		..Default::default()
    })
}
