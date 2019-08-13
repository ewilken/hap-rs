// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		active,
		name,
		status_fault,
	},
    HapType,
};

/// Faucet Service.
pub type Faucet = Service<FaucetInner>;

impl Default for Faucet {
    fn default() -> Faucet { new() }
}

/// Inner type of the Faucet Service.
#[derive(Default)]
pub struct FaucetInner {
    /// ID of the Faucet Service.
    id: u64,
    /// `HapType` of the Faucet Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Active Characteristic.
	pub active: active::Active,

	/// Name Characteristic.
	pub name: Option<name::Name>,
	/// Status Fault Characteristic.
	pub status_fault: Option<status_fault::StatusFault>,
}

impl HapService for FaucetInner {
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
		];
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &self.status_fault {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> {
        let mut characteristics: Vec<&mut HapCharacteristic> = vec![
			&mut self.active,
		];
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.status_fault {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Faucet Service.
pub fn new() -> Faucet {
    Faucet::new(FaucetInner {
        hap_type: HapType::Faucet,
		active: active::new(),
		..Default::default()
    })
}
