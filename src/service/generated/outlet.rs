// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		on,
		outlet_in_use,
		name,
	},
    HapType,
};

/// Outlet Service.
pub type Outlet = Service<OutletInner>;

impl Default for Outlet {
    fn default() -> Outlet { new() }
}

/// Inner type of the Outlet Service.
#[derive(Default)]
pub struct OutletInner {
    /// ID of the Outlet Service.
    id: u64,
    /// `HapType` of the Outlet Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// On Characteristic.
	pub on: on::On,
	/// Outlet In Use Characteristic.
	pub outlet_in_use: outlet_in_use::OutletInUse,

	/// Name Characteristic.
	pub name: Option<name::Name>,
}

impl HapService for OutletInner {
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
			&self.outlet_in_use,
		];
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> {
        let mut characteristics: Vec<&mut HapCharacteristic> = vec![
			&mut self.on,
			&mut self.outlet_in_use,
		];
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Outlet Service.
pub fn new() -> Outlet {
    Outlet::new(OutletInner {
        hap_type: HapType::Outlet,
		on: on::new(),
		outlet_in_use: outlet_in_use::new(),
		..Default::default()
    })
}
