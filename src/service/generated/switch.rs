// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		on,
		name,
	},
    HapType,
};

/// Switch Service.
pub type Switch = Service<SwitchInner>;

impl Default for Switch {
    fn default() -> Switch { new() }
}

/// Inner type of the Switch Service.
#[derive(Default)]
pub struct SwitchInner {
    /// ID of the Switch Service.
    id: u64,
    /// `HapType` of the Switch Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// On Characteristic.
	pub on: on::On,

	/// Name Characteristic.
	pub name: Option<name::Name>,
}

impl HapService for SwitchInner {
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
			&self.on,
		];
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> {
        let mut characteristics: Vec<&mut HapCharacteristic> = vec![
			&mut self.on,
		];
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Switch Service.
pub fn new() -> Switch {
    Switch::new(SwitchInner {
        hap_type: HapType::Switch,
		on: on::new(),
		..Default::default()
    })
}
