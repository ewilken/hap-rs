// THIS FILE IS AUTO-GENERATED

use crate::{
    characteristic::{
        configured_name,
        current_visibility_state,
        identifier,
        input_device_type,
        input_source_type,
        is_configured,
        name,
        target_visibility_state,
        HapCharacteristic,
    },
    service::{HapService, Service},
    HapType,
};

/// Input Source Service.
pub type InputSource = Service<InputSourceInner>;

impl Default for InputSource {
    fn default() -> InputSource { new() }
}

/// Inner type of the Input Source Service.
#[derive(Default)]
pub struct InputSourceInner {
    /// ID of the Input Source Service.
    id: u64,
    /// `HapType` of the Input Source Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

    /// Configured Name Characteristic.
    pub configured_name: configured_name::ConfiguredName,
    /// Input Source Type Characteristic.
    pub input_source_type: input_source_type::InputSourceType,
    /// Is Configured Characteristic.
    pub is_configured: is_configured::IsConfigured,
    /// Current Visibility State Characteristic.
    pub current_visibility_state: current_visibility_state::CurrentVisibilityState,

    /// Identifier Characteristic.
    pub identifier: Option<identifier::Identifier>,
    /// Input Device Type Characteristic.
    pub input_device_type: Option<input_device_type::InputDeviceType>,
    /// Target Visibility State Characteristic.
    pub target_visibility_state: Option<target_visibility_state::TargetVisibilityState>,
    /// Name Characteristic.
    pub name: Option<name::Name>,
}

impl HapService for InputSourceInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_type(&self) -> HapType { self.hap_type }

    fn get_hidden(&self) -> bool { self.hidden }

    fn set_hidden(&mut self, hidden: bool) { self.hidden = hidden; }

    fn get_primary(&self) -> bool { self.primary }

    fn set_primary(&mut self, primary: bool) { self.primary = primary; }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
            &self.configured_name,
            &self.input_source_type,
            &self.is_configured,
            &self.current_visibility_state,
        ];
        if let Some(c) = &self.identifier {
            characteristics.push(c);
        }
        if let Some(c) = &self.input_device_type {
            characteristics.push(c);
        }
        if let Some(c) = &self.target_visibility_state {
            characteristics.push(c);
        }
        if let Some(c) = &self.name {
            characteristics.push(c);
        }
        characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
            &mut self.configured_name,
            &mut self.input_source_type,
            &mut self.is_configured,
            &mut self.current_visibility_state,
        ];
        if let Some(c) = &mut self.identifier {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.input_device_type {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.target_visibility_state {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.name {
            characteristics.push(c);
        }
        characteristics
    }
}

/// Creates a new Input Source Service.
pub fn new() -> InputSource {
    InputSource::new(InputSourceInner {
        hap_type: HapType::InputSource,
        configured_name: configured_name::new(),
        input_source_type: input_source_type::new(),
        is_configured: is_configured::new(),
        current_visibility_state: current_visibility_state::new(),
        ..Default::default()
    })
}
