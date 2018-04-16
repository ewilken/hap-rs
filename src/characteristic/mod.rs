use std::io::{Error, ErrorKind};

use serde::{ser::{Serialize, Serializer, SerializeStruct}, Deserialize};
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

    fn_read: Option<Box<FnMut() -> Option<T>>>,
    fn_update: Option<Box<FnMut(&Option<T>, &T)>>,
}

impl<T: Default + Serialize> Characteristic<T> where for<'de> T: Deserialize<'de> {
    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    pub fn get_type(&self) -> &HapType {
        &self.hap_type
    }

    pub fn get_format(&self) -> &Format {
        &self.format
    }

    pub fn get_perms(&self) -> &Vec<Perm> {
        &self.perms
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn get_event_notifications(&self) -> Option<bool> {
        self.event_notifications
    }

    pub fn set_event_notifications(&mut self, event_notifications: bool) {
        self.event_notifications = Some(event_notifications);
    }

    pub fn get_value(&mut self) -> &Option<T> {
        if let Some(ref mut on_read) = self.fn_read {
            self.value = on_read();
        }

        &self.value
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

        if let Some(ref mut on_update) = self.fn_update {
            on_update(&self.value, &val);
        }
        self.value = Some(val);

        Ok(())
    }

    pub fn get_unit(&self) -> &Option<Unit> {
        &self.unit
    }

    pub fn get_max_value(&self) -> &Option<T> {
        &self.max_value
    }

    pub fn set_max_value(&mut self, val: T) {
        self.max_value = Some(val);
    }

    pub fn get_min_value(&self) -> &Option<T> {
        &self.min_value
    }

    pub fn set_min_value(&mut self, val: T) {
        self.min_value = Some(val);
    }

    pub fn get_step_value(&self) -> &Option<T> {
        &self.step_value
    }

    pub fn set_step_value(&mut self, val: T) {
        self.step_value = Some(val);
    }

    pub fn get_max_len(&self) -> Option<u16> {
        self.max_len
    }

    pub fn on_read(&mut self, on_read: Box<FnMut() -> Option<T>>) {
        self.fn_read = Some(on_read);
    }

    pub fn on_update(&mut self, on_update: Box<FnMut(&Option<T>, &T)>) {
        self.fn_update = Some(on_update);
    }
}

impl<T: Default + Serialize> Serialize for Characteristic<T> where for<'de> T: Deserialize<'de> {
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
    fn get_id(&self) -> u64;
    fn set_id(&mut self, id: u64);
    fn get_type(&self) -> &HapType;
    fn get_format(&self) -> &Format;
    fn get_perms(&self) -> &Vec<Perm>;
    fn get_event_notifications(&self) -> Option<bool>;
    fn set_event_notifications(&mut self, event_notifications: bool);
    fn get_value(&mut self) -> Option<serde_json::Value>;
    fn set_value(&mut self, value: serde_json::Value) -> Result<(), Error>;
    fn get_unit(&self) -> &Option<Unit>;
    fn get_max_value(&self) -> Option<serde_json::Value>;
    fn get_min_value(&self) -> Option<serde_json::Value>;
    fn get_step_value(&self) -> Option<serde_json::Value>;
    fn get_max_len(&self) -> Option<u16>;
}

serialize_trait_object!(HapCharacteristic);

impl<T: Default + Serialize> HapCharacteristic for Characteristic<T> where for<'de> T: Deserialize<'de> {
    fn get_id(&self) -> u64 {
        self.get_id()
    }

    fn set_id(&mut self, id: u64) {
        self.set_id(id)
    }

    fn get_type(&self) -> &HapType {
        self.get_type()
    }

    fn get_format(&self) -> &Format {
        self.get_format()
    }

    fn get_perms(&self) -> &Vec<Perm> {
        self.get_perms()
    }

    fn get_event_notifications(&self) -> Option<bool> {
        self.get_event_notifications()
    }

    fn set_event_notifications(&mut self, event_notifications: bool) {
        self.set_event_notifications(event_notifications);
    }

    fn get_value(&mut self) -> Option<serde_json::Value> {
        if let &Some(ref v) = self.get_value() {
            return Some(json!(v));
        }
        None
    }

    fn set_value(&mut self, value: serde_json::Value) -> Result<(), Error> {
        let v;
        // for some reason the iOS device is setting boolean values
        // either as a boolean or as an integer
        if self.format == Format::Bool && value.is_number() {
            let num_v: u8 = serde_json::from_value(value)?;
            if num_v == 0 {
                v = serde_json::from_value(json!(false))?;
            } else if num_v == 1 {
                v = serde_json::from_value(json!(true))?;
            } else {
                return Err(Error::new(ErrorKind::Other, "invalid value"));
            }
        } else {
            v = serde_json::from_value(value)?;
        }
        self.set_value(v)
    }

    fn get_unit(&self) -> &Option<Unit> {
        self.get_unit()
    }

    fn get_max_value(&self) -> Option<serde_json::Value> {
        if let &Some(ref v) = self.get_max_value() {
            return Some(json!(v));
        }
        None
    }

    fn get_min_value(&self) -> Option<serde_json::Value> {
        if let &Some(ref v) = self.get_min_value() {
            return Some(json!(v));
        }
        None
    }

    fn get_step_value(&self) -> Option<serde_json::Value> {
        if let &Some(ref v) = self.get_step_value() {
            return Some(json!(v));
        }
        None
    }

    fn get_max_len(&self) -> Option<u16> {
        self.get_max_len()
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Perm {
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

#[derive(Debug, Clone, Serialize)]
pub enum Unit {
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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Format {
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
