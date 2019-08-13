// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		active,
		in_use,
		valve_type,
		set_duration,
		remaining_duration,
		is_configured,
		service_label_index,
		status_fault,
		name,
	},
    HapType,
};

/// Valve Service.
pub type Valve = Service<ValveInner>;

impl Default for Valve {
    fn default() -> Valve { new() }
}

/// Inner type of the Valve Service.
#[derive(Default)]
pub struct ValveInner {
    /// ID of the Valve Service.
    id: u64,
    /// `HapType` of the Valve Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Active Characteristic.
	pub active: active::Active,
	/// In Use Characteristic.
	pub in_use: in_use::InUse,
	/// Valve Type Characteristic.
	pub valve_type: valve_type::ValveType,

	/// Set Duration Characteristic.
	pub set_duration: Option<set_duration::SetDuration>,
	/// Remaining Duration Characteristic.
	pub remaining_duration: Option<remaining_duration::RemainingDuration>,
	/// Is Configured Characteristic.
	pub is_configured: Option<is_configured::IsConfigured>,
	/// Service Label Index Characteristic.
	pub service_label_index: Option<service_label_index::ServiceLabelIndex>,
	/// Status Fault Characteristic.
	pub status_fault: Option<status_fault::StatusFault>,
	/// Name Characteristic.
	pub name: Option<name::Name>,
}

impl HapService for ValveInner {
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
			&self.active,
			&self.in_use,
			&self.valve_type,
		];
		if let Some(c) = &self.set_duration {
		    characteristics.push(c);
		}
		if let Some(c) = &self.remaining_duration {
		    characteristics.push(c);
		}
		if let Some(c) = &self.is_configured {
		    characteristics.push(c);
		}
		if let Some(c) = &self.service_label_index {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_fault {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> {
        let mut characteristics: Vec<&mut HapCharacteristic> = vec![
			&mut self.active,
			&mut self.in_use,
			&mut self.valve_type,
		];
		if let Some(c) = &mut self.set_duration {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.remaining_duration {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.is_configured {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.service_label_index {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_fault {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Valve Service.
pub fn new() -> Valve {
    Valve::new(ValveInner {
        hap_type: HapType::Valve,
		active: active::new(),
		in_use: in_use::new(),
		valve_type: valve_type::new(),
		..Default::default()
    })
}
