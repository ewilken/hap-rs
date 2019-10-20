// THIS FILE IS AUTO-GENERATED

use crate::{
    characteristic::{lock_current_state, lock_target_state, name, HapCharacteristic},
    service::{HapService, Service},
    HapType,
};

/// Lock Mechanism Service.
pub type LockMechanism = Service<LockMechanismInner>;

impl Default for LockMechanism {
    fn default() -> LockMechanism { new() }
}

/// Inner type of the Lock Mechanism Service.
#[derive(Default)]
pub struct LockMechanismInner {
    /// ID of the Lock Mechanism Service.
    id: u64,
    /// `HapType` of the Lock Mechanism Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

    /// Lock Current State Characteristic.
    pub lock_current_state: lock_current_state::LockCurrentState,
    /// Lock Target State Characteristic.
    pub lock_target_state: lock_target_state::LockTargetState,

    /// Name Characteristic.
    pub name: Option<name::Name>,
}

impl HapService for LockMechanismInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_type(&self) -> HapType { self.hap_type }

    fn get_hidden(&self) -> bool { self.hidden }

    fn set_hidden(&mut self, hidden: bool) { self.hidden = hidden; }

    fn get_primary(&self) -> bool { self.primary }

    fn set_primary(&mut self, primary: bool) { self.primary = primary; }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![&self.lock_current_state, &self.lock_target_state];
        if let Some(c) = &self.name {
            characteristics.push(c);
        }
        characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> =
            vec![&mut self.lock_current_state, &mut self.lock_target_state];
        if let Some(c) = &mut self.name {
            characteristics.push(c);
        }
        characteristics
    }
}

/// Creates a new Lock Mechanism Service.
pub fn new() -> LockMechanism {
    LockMechanism::new(LockMechanismInner {
        hap_type: HapType::LockMechanism,
        lock_current_state: lock_current_state::new(),
        lock_target_state: lock_target_state::new(),
        ..Default::default()
    })
}
