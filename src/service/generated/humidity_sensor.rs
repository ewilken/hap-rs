// THIS FILE IS AUTO-GENERATED

use crate::{
    characteristic::{
        current_relative_humidity,
        name,
        status_active,
        status_fault,
        status_low_battery,
        status_tampered,
        HapCharacteristic,
    },
    service::{HapService, Service},
    HapType,
};

/// Humidity Sensor Service.
pub type HumiditySensor = Service<HumiditySensorInner>;

impl Default for HumiditySensor {
    fn default() -> HumiditySensor { new() }
}

/// Inner type of the Humidity Sensor Service.
#[derive(Default)]
pub struct HumiditySensorInner {
    /// ID of the Humidity Sensor Service.
    id: u64,
    /// `HapType` of the Humidity Sensor Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

    /// Current Relative Humidity Characteristic.
    pub current_relative_humidity: current_relative_humidity::CurrentRelativeHumidity,

    /// Status Active Characteristic.
    pub status_active: Option<status_active::StatusActive>,
    /// Status Fault Characteristic.
    pub status_fault: Option<status_fault::StatusFault>,
    /// Status Tampered Characteristic.
    pub status_tampered: Option<status_tampered::StatusTampered>,
    /// Status Low Battery Characteristic.
    pub status_low_battery: Option<status_low_battery::StatusLowBattery>,
    /// Name Characteristic.
    pub name: Option<name::Name>,
}

impl HapService for HumiditySensorInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_type(&self) -> HapType { self.hap_type }

    fn get_hidden(&self) -> bool { self.hidden }

    fn set_hidden(&mut self, hidden: bool) { self.hidden = hidden; }

    fn get_primary(&self) -> bool { self.primary }

    fn set_primary(&mut self, primary: bool) { self.primary = primary; }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![&self.current_relative_humidity];
        if let Some(c) = &self.status_active {
            characteristics.push(c);
        }
        if let Some(c) = &self.status_fault {
            characteristics.push(c);
        }
        if let Some(c) = &self.status_tampered {
            characteristics.push(c);
        }
        if let Some(c) = &self.status_low_battery {
            characteristics.push(c);
        }
        if let Some(c) = &self.name {
            characteristics.push(c);
        }
        characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![&mut self.current_relative_humidity];
        if let Some(c) = &mut self.status_active {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.status_fault {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.status_tampered {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.status_low_battery {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.name {
            characteristics.push(c);
        }
        characteristics
    }
}

/// Creates a new Humidity Sensor Service.
pub fn new() -> HumiditySensor {
    HumiditySensor::new(HumiditySensorInner {
        hap_type: HapType::HumiditySensor,
        current_relative_humidity: current_relative_humidity::new(),
        ..Default::default()
    })
}
