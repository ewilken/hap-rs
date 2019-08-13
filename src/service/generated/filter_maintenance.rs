// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		filter_change_indication,
		filter_life_level,
		reset_filter_indication,
		name,
	},
    HapType,
};

/// Filter Maintenance Service.
pub type FilterMaintenance = Service<FilterMaintenanceInner>;

impl Default for FilterMaintenance {
    fn default() -> FilterMaintenance { new() }
}

/// Inner type of the Filter Maintenance Service.
#[derive(Default)]
pub struct FilterMaintenanceInner {
    /// ID of the Filter Maintenance Service.
    id: u64,
    /// `HapType` of the Filter Maintenance Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Filter Change Indication Characteristic.
	pub filter_change_indication: filter_change_indication::FilterChangeIndication,

	/// Filter Life Level Characteristic.
	pub filter_life_level: Option<filter_life_level::FilterLifeLevel>,
	/// Reset Filter Indication Characteristic.
	pub reset_filter_indication: Option<reset_filter_indication::ResetFilterIndication>,
	/// Name Characteristic.
	pub name: Option<name::Name>,
}

impl HapService for FilterMaintenanceInner {
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
			&self.filter_change_indication,
		];
		if let Some(c) = &self.filter_life_level {
		    characteristics.push(c);
		}
		if let Some(c) = &self.reset_filter_indication {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.filter_change_indication,
		];
		if let Some(c) = &mut self.filter_life_level {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.reset_filter_indication {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Filter Maintenance Service.
pub fn new() -> FilterMaintenance {
    FilterMaintenance::new(FilterMaintenanceInner {
        hap_type: HapType::FilterMaintenance,
		filter_change_indication: filter_change_indication::new(),
		..Default::default()
    })
}
