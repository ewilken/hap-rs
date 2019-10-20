// THIS FILE IS AUTO-GENERATED

use crate::{
    characteristic::{name, programmable_switch_event, service_label_index, HapCharacteristic},
    service::{HapService, Service},
    HapType,
};

/// Stateless Programmable Switch Service.
pub type StatelessProgrammableSwitch = Service<StatelessProgrammableSwitchInner>;

impl Default for StatelessProgrammableSwitch {
    fn default() -> StatelessProgrammableSwitch { new() }
}

/// Inner type of the Stateless Programmable Switch Service.
#[derive(Default)]
pub struct StatelessProgrammableSwitchInner {
    /// ID of the Stateless Programmable Switch Service.
    id: u64,
    /// `HapType` of the Stateless Programmable Switch Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

    /// Programmable Switch Event Characteristic.
    pub programmable_switch_event: programmable_switch_event::ProgrammableSwitchEvent,

    /// Name Characteristic.
    pub name: Option<name::Name>,
    /// Service Label Index Characteristic.
    pub service_label_index: Option<service_label_index::ServiceLabelIndex>,
}

impl HapService for StatelessProgrammableSwitchInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_type(&self) -> HapType { self.hap_type }

    fn get_hidden(&self) -> bool { self.hidden }

    fn set_hidden(&mut self, hidden: bool) { self.hidden = hidden; }

    fn get_primary(&self) -> bool { self.primary }

    fn set_primary(&mut self, primary: bool) { self.primary = primary; }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![&self.programmable_switch_event];
        if let Some(c) = &self.name {
            characteristics.push(c);
        }
        if let Some(c) = &self.service_label_index {
            characteristics.push(c);
        }
        characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![&mut self.programmable_switch_event];
        if let Some(c) = &mut self.name {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.service_label_index {
            characteristics.push(c);
        }
        characteristics
    }
}

/// Creates a new Stateless Programmable Switch Service.
pub fn new() -> StatelessProgrammableSwitch {
    StatelessProgrammableSwitch::new(StatelessProgrammableSwitchInner {
        hap_type: HapType::StatelessProgrammableSwitch,
        programmable_switch_event: programmable_switch_event::new(),
        ..Default::default()
    })
}
