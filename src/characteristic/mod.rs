use std::io::{Error, ErrorKind};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_json;
use erased_serde;

use hap_type::HapType;

pub mod firmware_revision;
pub mod identify;
pub mod manufacturer;
pub mod model;
pub mod name;
pub mod on;
pub mod outlet_in_use;
pub mod serial_number;

#[derive(Default)]
pub struct Characteristic<T: Default + Serialize> {
    id: u64,
    hap_type: HapType,
    format: Format,
    perms: Vec<Perm>,
    description: Option<String>,
    event_notifications: Option<bool>,

    value: Option<T>,
    unit: Option<Unit>,

    max_value: Option<T>,
    min_value: Option<T>,
    step_value: Option<T>,
    max_len: Option<u16>,
    max_data_len: Option<u32>,
    valid_values: Option<Vec<T>>,
    valid_values_range: Option<[T; 2]>,
}

impl<T: Default + Serialize> Characteristic<T> {
    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn set_value(&mut self, val: T) -> Result<(), Error> {
        /*if let Some(ref max) = self.max_value {
            if &val > max {
                return Err(Error::new(ErrorKind::Other, "value above max_value"));
            }
        }
        if let Some(ref min) = self.min_value {
            if &val < min {
                return Err(Error::new(ErrorKind::Other, "value below min_value"));
            }
        }*/

        self.value = Some(val);

        Ok(())
    }

    pub fn set_min_value(&mut self, val: T) {
        self.min_value = Some(val);
    }

    pub fn set_max_value(&mut self, val: T) {
        self.max_value = Some(val);
    }

    pub fn set_step_value(&mut self, val: T) {
        self.step_value = Some(val);
    }
}

impl<T: Default + Serialize> Serialize for Characteristic<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Characteristic", 15)?;
        state.serialize_field("iid", &self.id)?;
        state.serialize_field("type", &self.hap_type)?;
        state.serialize_field("format", &self.format)?;
        state.serialize_field("perms", &self.perms)?;
        if let Some(ref description) = self.description {
            state.serialize_field("description", description)?;
        }
        if let Some(ref event_notifications) = self.event_notifications {
            state.serialize_field("ev", event_notifications)?;
        }

        if self.perms.contains(&Perm::PairedRead) {
            state.serialize_field("value", &self.value)?;
        }
        if let Some(ref unit) = self.unit {
            state.serialize_field("unit", unit)?;
        }
        if let Some(ref max_value) = self.max_value {
            state.serialize_field("maxValue", max_value)?;
        }
        if let Some(ref min_value) = self.min_value {
            state.serialize_field("minValue", min_value)?;
        }
        if let Some(ref step_value) = self.step_value {
            state.serialize_field("minStep", step_value)?;
        }
        if let Some(ref max_len) = self.max_len {
            state.serialize_field("maxLen", max_len)?;
        }
        if let Some(ref max_data_len) = self.max_data_len {
            state.serialize_field("maxDataLen", max_data_len)?;
        }
        if let Some(ref valid_values) = self.valid_values {
            state.serialize_field("valid-values", valid_values)?;
        }
        if let Some(ref valid_values_range) = self.valid_values_range {
            state.serialize_field("valid-values-range", valid_values_range)?;
        }
        state.end()
    }
}

pub trait HapCharacteristic: erased_serde::Serialize {
    fn set_id(&mut self, id: u64);
}

serialize_trait_object!(HapCharacteristic);

impl<T: Default + Serialize> HapCharacteristic for Characteristic<T> {
    fn set_id(&mut self, id: u64) {
        self.set_id(id)
    }
}

#[derive(Serialize, PartialEq)]
enum Perm {
    #[serde(rename = "pr")]
    PairedRead,
    #[serde(rename = "pw")]
    PairedWrite,
    #[serde(rename = "ev")]
    Events,
    #[serde(rename = "aa")]
    AdditionalAuthorization,
    #[serde(rename = "tw")]
    TimedWrite,
    #[serde(rename = "hd")]
    Hidden,
}

#[derive(Serialize)]
enum Unit {
    #[serde(rename = "percentage")]
    Percentage,
    #[serde(rename = "arcdegrees")]
    ArcDegrees,
    #[serde(rename = "celsius")]
    Celsius,
    #[serde(rename = "lux")]
    Lux,
    #[serde(rename = "seconds")]
    Seconds,
}

#[derive(Serialize)]
enum Format {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "uint8")]
    UInt8,
    #[serde(rename = "uint16")]
    UInt16,
    #[serde(rename = "uint32")]
    UInt32,
    #[serde(rename = "int32")]
    Int32,
    #[serde(rename = "uint64")]
    UInt64,
    #[serde(rename = "data")]
    Data,
    #[serde(rename = "tlv8")]
    TLV8,
}

impl Default for Format {
    fn default() -> Format {
        Format::String
    }
}
