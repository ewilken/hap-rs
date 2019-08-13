// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		current_position,
		position_state,
		target_position,
		hold_position,
		obstruction_detected,
		name,
	},
    HapType,
};

/// Door Service.
pub type Door = Service<DoorInner>;

impl Default for Door {
    fn default() -> Door { new() }
}

/// Inner type of the Door Service.
#[derive(Default)]
pub struct DoorInner {
    /// ID of the Door Service.
    id: u64,
    /// `HapType` of the Door Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Current Position Characteristic.
	pub current_position: current_position::CurrentPosition,
	/// Position State Characteristic.
	pub position_state: position_state::PositionState,
	/// Target Position Characteristic.
	pub target_position: target_position::TargetPosition,

	/// Hold Position Characteristic.
	pub hold_position: Option<hold_position::HoldPosition>,
	/// Obstruction Detected Characteristic.
	pub obstruction_detected: Option<obstruction_detected::ObstructionDetected>,
	/// Name Characteristic.
	pub name: Option<name::Name>,
}

impl HapService for DoorInner {
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
			&self.current_position,
			&self.position_state,
			&self.target_position,
		];
		if let Some(c) = &self.hold_position {
		    characteristics.push(c);
		}
		if let Some(c) = &self.obstruction_detected {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.current_position,
			&mut self.position_state,
			&mut self.target_position,
		];
		if let Some(c) = &mut self.hold_position {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.obstruction_detected {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Door Service.
pub fn new() -> Door {
    Door::new(DoorInner {
        hap_type: HapType::Door,
		current_position: current_position::new(),
		position_state: position_state::new(),
		target_position: target_position::new(),
		..Default::default()
    })
}
