// THIS FILE IS AUTO-GENERATED

use crate::{
    characteristic::{
        active,
        active_identifier,
        brightness,
        closed_captions,
        configured_name,
        current_media_state,
        display_order,
        picture_mode,
        power_mode_selection,
        remote_key,
        sleep_discovery_mode,
        target_media_state,
        HapCharacteristic,
    },
    service::{HapService, Service},
    HapType,
};

/// Television Service.
pub type Television = Service<TelevisionInner>;

impl Default for Television {
    fn default() -> Television { new() }
}

/// Inner type of the Television Service.
#[derive(Default)]
pub struct TelevisionInner {
    /// ID of the Television Service.
    id: u64,
    /// `HapType` of the Television Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

    /// Active Characteristic.
    pub active: active::Active,
    /// Active Identifier Characteristic.
    pub active_identifier: active_identifier::ActiveIdentifier,
    /// Configured Name Characteristic.
    pub configured_name: configured_name::ConfiguredName,
    /// Sleep Discovery Mode Characteristic.
    pub sleep_discovery_mode: sleep_discovery_mode::SleepDiscoveryMode,

    /// Brightness Characteristic.
    pub brightness: Option<brightness::Brightness>,
    /// Closed Captions Characteristic.
    pub closed_captions: Option<closed_captions::ClosedCaptions>,
    /// Display Order Characteristic.
    pub display_order: Option<display_order::DisplayOrder>,
    /// Current Media State Characteristic.
    pub current_media_state: Option<current_media_state::CurrentMediaState>,
    /// Target Media State Characteristic.
    pub target_media_state: Option<target_media_state::TargetMediaState>,
    /// Picture Mode Characteristic.
    pub picture_mode: Option<picture_mode::PictureMode>,
    /// Power Mode Selection Characteristic.
    pub power_mode_selection: Option<power_mode_selection::PowerModeSelection>,
    /// Remote Key Characteristic.
    pub remote_key: Option<remote_key::RemoteKey>,
}

impl HapService for TelevisionInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_type(&self) -> HapType { self.hap_type }

    fn get_hidden(&self) -> bool { self.hidden }

    fn set_hidden(&mut self, hidden: bool) { self.hidden = hidden; }

    fn get_primary(&self) -> bool { self.primary }

    fn set_primary(&mut self, primary: bool) { self.primary = primary; }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
            &self.active,
            &self.active_identifier,
            &self.configured_name,
            &self.sleep_discovery_mode,
        ];
        if let Some(c) = &self.brightness {
            characteristics.push(c);
        }
        if let Some(c) = &self.closed_captions {
            characteristics.push(c);
        }
        if let Some(c) = &self.display_order {
            characteristics.push(c);
        }
        if let Some(c) = &self.current_media_state {
            characteristics.push(c);
        }
        if let Some(c) = &self.target_media_state {
            characteristics.push(c);
        }
        if let Some(c) = &self.picture_mode {
            characteristics.push(c);
        }
        if let Some(c) = &self.power_mode_selection {
            characteristics.push(c);
        }
        if let Some(c) = &self.remote_key {
            characteristics.push(c);
        }
        characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
            &mut self.active,
            &mut self.active_identifier,
            &mut self.configured_name,
            &mut self.sleep_discovery_mode,
        ];
        if let Some(c) = &mut self.brightness {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.closed_captions {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.display_order {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.current_media_state {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.target_media_state {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.picture_mode {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.power_mode_selection {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.remote_key {
            characteristics.push(c);
        }
        characteristics
    }
}

/// Creates a new Television Service.
pub fn new() -> Television {
    Television::new(TelevisionInner {
        hap_type: HapType::Television,
        active: active::new(),
        active_identifier: active_identifier::new(),
        configured_name: configured_name::new(),
        sleep_discovery_mode: sleep_discovery_mode::new(),
        ..Default::default()
    })
}
