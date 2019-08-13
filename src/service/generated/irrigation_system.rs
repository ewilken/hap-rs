// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		active,
		program_mode,
		in_use,
		name,
		remaining_duration,
		status_fault,
	},
    HapType,
};

/// Irrigation System Service.
pub type IrrigationSystem = Service<IrrigationSystemInner>;

impl Default for IrrigationSystem {
    fn default() -> IrrigationSystem { new() }
}

/// Inner type of the Irrigation System Service.
#[derive(Default)]
pub struct IrrigationSystemInner {
    /// ID of the Irrigation System Service.
    id: u64,
    /// `HapType` of the Irrigation System Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Active Characteristic.
	pub active: active::Active,
	/// Program Mode Characteristic.
	pub program_mode: program_mode::ProgramMode,
	/// In Use Characteristic.
	pub in_use: in_use::InUse,

	/// Name Characteristic.
	pub name: Option<name::Name>,
	/// Remaining Duration Characteristic.
	pub remaining_duration: Option<remaining_duration::RemainingDuration>,
	/// Status Fault Characteristic.
	pub status_fault: Option<status_fault::StatusFault>,
}

impl HapService for IrrigationSystemInner {
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

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
			&self.active,
			&self.program_mode,
			&self.in_use,
		];
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &self.remaining_duration {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_fault {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.active,
			&mut self.program_mode,
			&mut self.in_use,
		];
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.remaining_duration {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_fault {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Irrigation System Service.
pub fn new() -> IrrigationSystem {
    IrrigationSystem::new(IrrigationSystemInner {
        hap_type: HapType::IrrigationSystem,
		active: active::new(),
		program_mode: program_mode::new(),
		in_use: in_use::new(),
		..Default::default()
    })
}
