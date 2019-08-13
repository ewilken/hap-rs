// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		current_position,
		target_position,
		position_state,
		hold_position,
		target_horizontal_tilt_angle,
		target_vertical_tilt_angle,
		current_horizontal_tilt_angle,
		current_vertical_tilt_angle,
		obstruction_detected,
		name,
	},
    HapType,
};

/// Window Covering Service.
pub type WindowCovering = Service<WindowCoveringInner>;

impl Default for WindowCovering {
    fn default() -> WindowCovering { new() }
}

/// Inner type of the Window Covering Service.
#[derive(Default)]
pub struct WindowCoveringInner {
    /// ID of the Window Covering Service.
    id: u64,
    /// `HapType` of the Window Covering Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Current Position Characteristic.
	pub current_position: current_position::CurrentPosition,
	/// Target Position Characteristic.
	pub target_position: target_position::TargetPosition,
	/// Position State Characteristic.
	pub position_state: position_state::PositionState,

	/// Hold Position Characteristic.
	pub hold_position: Option<hold_position::HoldPosition>,
	/// Target Horizontal Tilt Angle Characteristic.
	pub target_horizontal_tilt_angle: Option<target_horizontal_tilt_angle::TargetHorizontalTiltAngle>,
	/// Target Vertical Tilt Angle Characteristic.
	pub target_vertical_tilt_angle: Option<target_vertical_tilt_angle::TargetVerticalTiltAngle>,
	/// Current Horizontal Tilt Angle Characteristic.
	pub current_horizontal_tilt_angle: Option<current_horizontal_tilt_angle::CurrentHorizontalTiltAngle>,
	/// Current Vertical Tilt Angle Characteristic.
	pub current_vertical_tilt_angle: Option<current_vertical_tilt_angle::CurrentVerticalTiltAngle>,
	/// Obstruction Detected Characteristic.
	pub obstruction_detected: Option<obstruction_detected::ObstructionDetected>,
	/// Name Characteristic.
	pub name: Option<name::Name>,
}

impl HapService for WindowCoveringInner {
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
			&self.target_position,
			&self.position_state,
		];
		if let Some(c) = &self.hold_position {
		    characteristics.push(c);
		}
		if let Some(c) = &self.target_horizontal_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &self.target_vertical_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &self.current_horizontal_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &self.current_vertical_tilt_angle {
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
			&mut self.target_position,
			&mut self.position_state,
		];
		if let Some(c) = &mut self.hold_position {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.target_horizontal_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.target_vertical_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.current_horizontal_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.current_vertical_tilt_angle {
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

/// Creates a new Window Covering Service.
pub fn new() -> WindowCovering {
    WindowCovering::new(WindowCoveringInner {
        hap_type: HapType::WindowCovering,
		current_position: current_position::new(),
		target_position: target_position::new(),
		position_state: position_state::new(),
		..Default::default()
    })
}
