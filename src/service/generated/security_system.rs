// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		security_system_current_state,
		security_system_target_state,
		status_fault,
		status_tampered,
		security_system_alarm_type,
		name,
	},
    HapType,
};

/// Security System Service.
pub type SecuritySystem = Service<SecuritySystemInner>;

impl Default for SecuritySystem {
    fn default() -> SecuritySystem { new() }
}

/// Inner type of the Security System Service.
#[derive(Default)]
pub struct SecuritySystemInner {
    /// ID of the Security System Service.
    id: u64,
    /// `HapType` of the Security System Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Security System Current State Characteristic.
	pub security_system_current_state: security_system_current_state::SecuritySystemCurrentState,
	/// Security System Target State Characteristic.
	pub security_system_target_state: security_system_target_state::SecuritySystemTargetState,

	/// Status Fault Characteristic.
	pub status_fault: Option<status_fault::StatusFault>,
	/// Status Tampered Characteristic.
	pub status_tampered: Option<status_tampered::StatusTampered>,
	/// Security System Alarm Type Characteristic.
	pub security_system_alarm_type: Option<security_system_alarm_type::SecuritySystemAlarmType>,
	/// Name Characteristic.
	pub name: Option<name::Name>,
}

impl HapService for SecuritySystemInner {
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

    fn get_characteristics(&self) -> Vec<&HapCharacteristic> {
        let mut characteristics: Vec<&HapCharacteristic> = vec![
			&self.security_system_current_state,
			&self.security_system_target_state,
		];
		if let Some(c) = &self.status_fault {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_tampered {
		    characteristics.push(c);
		}
		if let Some(c) = &self.security_system_alarm_type {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> {
        let mut characteristics: Vec<&mut HapCharacteristic> = vec![
			&mut self.security_system_current_state,
			&mut self.security_system_target_state,
		];
		if let Some(c) = &mut self.status_fault {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_tampered {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.security_system_alarm_type {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Security System Service.
pub fn new() -> SecuritySystem {
    SecuritySystem::new(SecuritySystemInner {
        hap_type: HapType::SecuritySystem,
		security_system_current_state: security_system_current_state::new(),
		security_system_target_state: security_system_target_state::new(),
		..Default::default()
    })
}
