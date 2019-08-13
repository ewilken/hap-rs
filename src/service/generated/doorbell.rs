// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		programmable_switch_event,
		brightness,
		volume,
		name,
	},
    HapType,
};

/// Doorbell Service.
pub type Doorbell = Service<DoorbellInner>;

impl Default for Doorbell {
    fn default() -> Doorbell { new() }
}

/// Inner type of the Doorbell Service.
#[derive(Default)]
pub struct DoorbellInner {
    /// ID of the Doorbell Service.
    id: u64,
    /// `HapType` of the Doorbell Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Programmable Switch Event Characteristic.
	pub programmable_switch_event: programmable_switch_event::ProgrammableSwitchEvent,

	/// Brightness Characteristic.
	pub brightness: Option<brightness::Brightness>,
	/// Volume Characteristic.
	pub volume: Option<volume::Volume>,
	/// Name Characteristic.
	pub name: Option<name::Name>,
}

impl HapService for DoorbellInner {
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
			&self.programmable_switch_event,
		];
		if let Some(c) = &self.brightness {
		    characteristics.push(c);
		}
		if let Some(c) = &self.volume {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> {
        let mut characteristics: Vec<&mut HapCharacteristic> = vec![
			&mut self.programmable_switch_event,
		];
		if let Some(c) = &mut self.brightness {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.volume {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Doorbell Service.
pub fn new() -> Doorbell {
    Doorbell::new(DoorbellInner {
        hap_type: HapType::Doorbell,
		programmable_switch_event: programmable_switch_event::new(),
		..Default::default()
    })
}
