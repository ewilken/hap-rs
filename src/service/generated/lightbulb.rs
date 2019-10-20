// THIS FILE IS AUTO-GENERATED

use crate::{
    characteristic::{brightness, hue, name, on, saturation, HapCharacteristic},
    service::{HapService, Service},
    HapType,
};

/// Lightbulb Service.
pub type Lightbulb = Service<LightbulbInner>;

impl Default for Lightbulb {
    fn default() -> Lightbulb { new() }
}

/// Inner type of the Lightbulb Service.
#[derive(Default)]
pub struct LightbulbInner {
    /// ID of the Lightbulb Service.
    id: u64,
    /// `HapType` of the Lightbulb Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

    /// On Characteristic.
    pub on: on::On,

    /// Brightness Characteristic.
    pub brightness: Option<brightness::Brightness>,
    /// Hue Characteristic.
    pub hue: Option<hue::Hue>,
    /// Saturation Characteristic.
    pub saturation: Option<saturation::Saturation>,
    /// Name Characteristic.
    pub name: Option<name::Name>,
}

impl HapService for LightbulbInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_type(&self) -> HapType { self.hap_type }

    fn get_hidden(&self) -> bool { self.hidden }

    fn set_hidden(&mut self, hidden: bool) { self.hidden = hidden; }

    fn get_primary(&self) -> bool { self.primary }

    fn set_primary(&mut self, primary: bool) { self.primary = primary; }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![&self.on];
        if let Some(c) = &self.brightness {
            characteristics.push(c);
        }
        if let Some(c) = &self.hue {
            characteristics.push(c);
        }
        if let Some(c) = &self.saturation {
            characteristics.push(c);
        }
        if let Some(c) = &self.name {
            characteristics.push(c);
        }
        characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![&mut self.on];
        if let Some(c) = &mut self.brightness {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.hue {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.saturation {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.name {
            characteristics.push(c);
        }
        characteristics
    }
}

/// Creates a new Lightbulb Service.
pub fn new() -> Lightbulb {
    Lightbulb::new(LightbulbInner {
        hap_type: HapType::Lightbulb,
        on: on::new(),
        ..Default::default()
    })
}
