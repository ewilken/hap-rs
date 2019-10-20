// THIS FILE IS AUTO-GENERATED

use crate::{
    characteristic::{name, on, rotation_direction, rotation_speed, HapCharacteristic},
    service::{HapService, Service},
    HapType,
};

/// Fan Service.
pub type Fan = Service<FanInner>;

impl Default for Fan {
    fn default() -> Fan { new() }
}

/// Inner type of the Fan Service.
#[derive(Default)]
pub struct FanInner {
    /// ID of the Fan Service.
    id: u64,
    /// `HapType` of the Fan Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

    /// On Characteristic.
    pub on: on::On,

    /// Rotation Direction Characteristic.
    pub rotation_direction: Option<rotation_direction::RotationDirection>,
    /// Rotation Speed Characteristic.
    pub rotation_speed: Option<rotation_speed::RotationSpeed>,
    /// Name Characteristic.
    pub name: Option<name::Name>,
}

impl HapService for FanInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_type(&self) -> HapType { self.hap_type }

    fn get_hidden(&self) -> bool { self.hidden }

    fn set_hidden(&mut self, hidden: bool) { self.hidden = hidden; }

    fn get_primary(&self) -> bool { self.primary }

    fn set_primary(&mut self, primary: bool) { self.primary = primary; }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![&self.on];
        if let Some(c) = &self.rotation_direction {
            characteristics.push(c);
        }
        if let Some(c) = &self.rotation_speed {
            characteristics.push(c);
        }
        if let Some(c) = &self.name {
            characteristics.push(c);
        }
        characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![&mut self.on];
        if let Some(c) = &mut self.rotation_direction {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.rotation_speed {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.name {
            characteristics.push(c);
        }
        characteristics
    }
}

/// Creates a new Fan Service.
pub fn new() -> Fan {
    Fan::new(FanInner {
        hap_type: HapType::Fan,
        on: on::new(),
        ..Default::default()
    })
}
