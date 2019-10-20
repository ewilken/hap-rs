// THIS FILE IS AUTO-GENERATED

use crate::{
    characteristic::{battery_level, charging_state, name, status_low_battery, HapCharacteristic},
    service::{HapService, Service},
    HapType,
};

/// Battery Service Service.
pub type BatteryService = Service<BatteryServiceInner>;

impl Default for BatteryService {
    fn default() -> BatteryService { new() }
}

/// Inner type of the Battery Service Service.
#[derive(Default)]
pub struct BatteryServiceInner {
    /// ID of the Battery Service Service.
    id: u64,
    /// `HapType` of the Battery Service Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

    /// Battery Level Characteristic.
    pub battery_level: battery_level::BatteryLevel,
    /// Charging State Characteristic.
    pub charging_state: charging_state::ChargingState,
    /// Status Low Battery Characteristic.
    pub status_low_battery: status_low_battery::StatusLowBattery,

    /// Name Characteristic.
    pub name: Option<name::Name>,
}

impl HapService for BatteryServiceInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_type(&self) -> HapType { self.hap_type }

    fn get_hidden(&self) -> bool { self.hidden }

    fn set_hidden(&mut self, hidden: bool) { self.hidden = hidden; }

    fn get_primary(&self) -> bool { self.primary }

    fn set_primary(&mut self, primary: bool) { self.primary = primary; }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> =
            vec![&self.battery_level, &self.charging_state, &self.status_low_battery];
        if let Some(c) = &self.name {
            characteristics.push(c);
        }
        characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
            &mut self.battery_level,
            &mut self.charging_state,
            &mut self.status_low_battery,
        ];
        if let Some(c) = &mut self.name {
            characteristics.push(c);
        }
        characteristics
    }
}

/// Creates a new Battery Service Service.
pub fn new() -> BatteryService {
    BatteryService::new(BatteryServiceInner {
        hap_type: HapType::BatteryService,
        battery_level: battery_level::new(),
        charging_state: charging_state::new(),
        status_low_battery: status_low_battery::new(),
        ..Default::default()
    })
}
