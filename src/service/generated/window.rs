// THIS FILE IS AUTO-GENERATED

use crate::{
    characteristic::{
        current_position,
        hold_position,
        name,
        obstruction_detected,
        position_state,
        target_position,
        HapCharacteristic,
    },
    service::{HapService, Service},
    HapType,
};

/// Window Service.
pub type Window = Service<WindowInner>;

impl Default for Window {
    fn default() -> Window { new() }
}

/// Inner type of the Window Service.
#[derive(Default)]
pub struct WindowInner {
    /// ID of the Window Service.
    id: u64,
    /// `HapType` of the Window Service.
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
    /// Obstruction Detected Characteristic.
    pub obstruction_detected: Option<obstruction_detected::ObstructionDetected>,
    /// Name Characteristic.
    pub name: Option<name::Name>,
}

impl HapService for WindowInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_type(&self) -> HapType { self.hap_type }

    fn get_hidden(&self) -> bool { self.hidden }

    fn set_hidden(&mut self, hidden: bool) { self.hidden = hidden; }

    fn get_primary(&self) -> bool { self.primary }

    fn set_primary(&mut self, primary: bool) { self.primary = primary; }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> =
            vec![&self.current_position, &self.target_position, &self.position_state];
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
            &mut self.target_position,
            &mut self.position_state,
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

/// Creates a new Window Service.
pub fn new() -> Window {
    Window::new(WindowInner {
        hap_type: HapType::Window,
        current_position: current_position::new(),
        target_position: target_position::new(),
        position_state: position_state::new(),
        ..Default::default()
    })
}
