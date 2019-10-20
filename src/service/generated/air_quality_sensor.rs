// THIS FILE IS AUTO-GENERATED

use crate::{
    characteristic::{
        air_quality,
        carbon_dioxide_level,
        carbon_monoxide_level,
        name,
        nitrogen_dioxide_density,
        ozone_density,
        pm10_density,
        pm2_5_density,
        status_active,
        status_fault,
        status_low_battery,
        status_tampered,
        sulphur_dioxide_density,
        voc_density,
        HapCharacteristic,
    },
    service::{HapService, Service},
    HapType,
};

/// Air Quality Sensor Service.
pub type AirQualitySensor = Service<AirQualitySensorInner>;

impl Default for AirQualitySensor {
    fn default() -> AirQualitySensor { new() }
}

/// Inner type of the Air Quality Sensor Service.
#[derive(Default)]
pub struct AirQualitySensorInner {
    /// ID of the Air Quality Sensor Service.
    id: u64,
    /// `HapType` of the Air Quality Sensor Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

    /// Air Quality Characteristic.
    pub air_quality: air_quality::AirQuality,

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
    /// Ozone Density Characteristic.
    pub ozone_density: Option<ozone_density::OzoneDensity>,
    /// Nitrogen Dioxide Density Characteristic.
    pub nitrogen_dioxide_density: Option<nitrogen_dioxide_density::NitrogenDioxideDensity>,
    /// Sulphur Dioxide Density Characteristic.
    pub sulphur_dioxide_density: Option<sulphur_dioxide_density::SulphurDioxideDensity>,
    /// PM2.5 Density Characteristic.
    pub pm2_5_density: Option<pm2_5_density::PM2_5Density>,
    /// PM10 Density Characteristic.
    pub pm10_density: Option<pm10_density::PM10Density>,
    /// VOC Density Characteristic.
    pub voc_density: Option<voc_density::VOCDensity>,
    /// Carbon Monoxide Level Characteristic.
    pub carbon_monoxide_level: Option<carbon_monoxide_level::CarbonMonoxideLevel>,
    /// Carbon Dioxide Level Characteristic.
    pub carbon_dioxide_level: Option<carbon_dioxide_level::CarbonDioxideLevel>,
}

impl HapService for AirQualitySensorInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_type(&self) -> HapType { self.hap_type }

    fn get_hidden(&self) -> bool { self.hidden }

    fn set_hidden(&mut self, hidden: bool) { self.hidden = hidden; }

    fn get_primary(&self) -> bool { self.primary }

    fn set_primary(&mut self, primary: bool) { self.primary = primary; }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![&self.air_quality];
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
        if let Some(c) = &self.ozone_density {
            characteristics.push(c);
        }
        if let Some(c) = &self.nitrogen_dioxide_density {
            characteristics.push(c);
        }
        if let Some(c) = &self.sulphur_dioxide_density {
            characteristics.push(c);
        }
        if let Some(c) = &self.pm2_5_density {
            characteristics.push(c);
        }
        if let Some(c) = &self.pm10_density {
            characteristics.push(c);
        }
        if let Some(c) = &self.voc_density {
            characteristics.push(c);
        }
        if let Some(c) = &self.carbon_monoxide_level {
            characteristics.push(c);
        }
        if let Some(c) = &self.carbon_dioxide_level {
            characteristics.push(c);
        }
        characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![&mut self.air_quality];
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
        if let Some(c) = &mut self.ozone_density {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.nitrogen_dioxide_density {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.sulphur_dioxide_density {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.pm2_5_density {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.pm10_density {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.voc_density {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.carbon_monoxide_level {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.carbon_dioxide_level {
            characteristics.push(c);
        }
        characteristics
    }
}

/// Creates a new Air Quality Sensor Service.
pub fn new() -> AirQualitySensor {
    AirQualitySensor::new(AirQualitySensorInner {
        hap_type: HapType::AirQualitySensor,
        air_quality: air_quality::new(),
        ..Default::default()
    })
}
