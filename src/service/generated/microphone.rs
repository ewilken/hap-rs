// THIS FILE IS AUTO-GENERATED

use crate::{
    characteristic::{mute, name, volume, HapCharacteristic},
    service::{HapService, Service},
    HapType,
};

/// Microphone Service.
pub type Microphone = Service<MicrophoneInner>;

impl Default for Microphone {
    fn default() -> Microphone { new() }
}

/// Inner type of the Microphone Service.
#[derive(Default)]
pub struct MicrophoneInner {
    /// ID of the Microphone Service.
    id: u64,
    /// `HapType` of the Microphone Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

    /// Volume Characteristic.
    pub volume: volume::Volume,
    /// Mute Characteristic.
    pub mute: mute::Mute,

    /// Name Characteristic.
    pub name: Option<name::Name>,
}

impl HapService for MicrophoneInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_type(&self) -> HapType { self.hap_type }

    fn get_hidden(&self) -> bool { self.hidden }

    fn set_hidden(&mut self, hidden: bool) { self.hidden = hidden; }

    fn get_primary(&self) -> bool { self.primary }

    fn set_primary(&mut self, primary: bool) { self.primary = primary; }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![&self.volume, &self.mute];
        if let Some(c) = &self.name {
            characteristics.push(c);
        }
        characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![&mut self.volume, &mut self.mute];
        if let Some(c) = &mut self.name {
            characteristics.push(c);
        }
        characteristics
    }
}

/// Creates a new Microphone Service.
pub fn new() -> Microphone {
    Microphone::new(MicrophoneInner {
        hap_type: HapType::Microphone,
        volume: volume::new(),
        mute: mute::new(),
        ..Default::default()
    })
}
