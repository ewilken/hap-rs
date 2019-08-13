// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		current_door_state,
		target_door_state,
		obstruction_detected,
		lock_current_state,
		lock_target_state,
		name,
	},
    HapType,
};

/// Garage Door Opener Service.
pub type GarageDoorOpener = Service<GarageDoorOpenerInner>;

impl Default for GarageDoorOpener {
    fn default() -> GarageDoorOpener { new() }
}

/// Inner type of the Garage Door Opener Service.
#[derive(Default)]
pub struct GarageDoorOpenerInner {
    /// ID of the Garage Door Opener Service.
    id: u64,
    /// `HapType` of the Garage Door Opener Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Current Door State Characteristic.
	pub current_door_state: current_door_state::CurrentDoorState,
	/// Target Door State Characteristic.
	pub target_door_state: target_door_state::TargetDoorState,
	/// Obstruction Detected Characteristic.
	pub obstruction_detected: obstruction_detected::ObstructionDetected,

	/// Lock Current State Characteristic.
	pub lock_current_state: Option<lock_current_state::LockCurrentState>,
	/// Lock Target State Characteristic.
	pub lock_target_state: Option<lock_target_state::LockTargetState>,
	/// Name Characteristic.
	pub name: Option<name::Name>,
}

impl HapService for GarageDoorOpenerInner {
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
			&self.current_door_state,
			&self.target_door_state,
			&self.obstruction_detected,
		];
		if let Some(c) = &self.lock_current_state {
		    characteristics.push(c);
		}
		if let Some(c) = &self.lock_target_state {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.current_door_state,
			&mut self.target_door_state,
			&mut self.obstruction_detected,
		];
		if let Some(c) = &mut self.lock_current_state {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.lock_target_state {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Garage Door Opener Service.
pub fn new() -> GarageDoorOpener {
    GarageDoorOpener::new(GarageDoorOpenerInner {
        hap_type: HapType::GarageDoorOpener,
		current_door_state: current_door_state::new(),
		target_door_state: target_door_state::new(),
		obstruction_detected: obstruction_detected::new(),
		..Default::default()
    })
}
