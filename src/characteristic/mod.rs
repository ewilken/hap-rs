use std::fmt;

use async_trait::async_trait;
use erased_serde::serialize_trait_object;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize,
    Serialize,
};
use serde_json::json;

use crate::{event::Event, pointer, HapType, Result};

mod generated;

pub use generated::*;

/// A Characteristic. A characteristic is a feature that represents data or an associated behavior of a service. The
/// characteristic is defined by a universally unique type, and has additional properties that determine how the value
/// of the characteristic can be accessed.
#[derive(Debug, Default)]
pub struct Characteristic<T: Default + Clone + Serialize + Send + Sync> {
    id: u64,
    accessory_id: u64,
    hap_type: HapType,
    format: Format,
    perms: Vec<Perm>,
    description: Option<String>,
    event_notifications: Option<bool>,

    value: T,
    unit: Option<Unit>,

    max_value: Option<T>,
    min_value: Option<T>,
    step_value: Option<T>,
    max_len: Option<u16>,
    max_data_len: Option<u32>,
    valid_values: Option<Vec<T>>,
    valid_values_range: Option<[T; 2]>,

    on_read: Option<Box<dyn OnReadFn<T>>>,
    on_update: Option<Box<dyn OnUpdateFn<T>>>,

    event_emitter: Option<pointer::EventEmitter>,
}

impl<T: Default + Clone + Serialize + Send + Sync> Characteristic<T>
where
    for<'de> T: Deserialize<'de>,
{
    /// Returns the ID of a Characteristic.
    pub fn get_id(&self) -> u64 { self.id }

    /// Returns the `HapType` of a Characteristic.
    pub fn get_type(&self) -> HapType { self.hap_type }

    /// Returns the `Format` of a Characteristic.
    pub fn get_format(&self) -> Format { self.format }

    /// Returns the `Perm`s of a Characteristic.
    pub fn get_perms(&self) -> Vec<Perm> { self.perms.clone() }

    /// Sets the description of a Characteristic.
    pub fn set_description(&mut self, description: Option<String>) { self.description = description; }

    /// Returns the event notifications value of a Characteristic.
    pub fn get_event_notifications(&self) -> Option<bool> { self.event_notifications }

    /// Sets the event notifications value of a Characteristic.
    pub fn set_event_notifications(&mut self, event_notifications: Option<bool>) {
        self.event_notifications = event_notifications;
    }

    /// Returns the value of a Characteristic.
    pub async fn get_value(&mut self) -> Result<T> {
        let mut val = None;
        if let Some(ref on_read) = self.on_read {
            val = on_read();
        }
        if let Some(v) = val {
            self.set_value(v).await?;
        }

        Ok(self.value.clone())
    }

    /// Sets the value of a Characteristic.
    pub async fn set_value(&mut self, val: T) -> Result<()> {
        // TODO: check for min/max on types implementing PartialOrd
        // if let Some(ref max) = self.inner.try_borrow()?.max_value {
        //     if &val > max {
        //         return Err(Error::from_str("value above max_value"));
        //     }
        // }
        // if let Some(ref min) = self.inner.try_borrow()?.min_value {
        //     if &val < min {
        //         return Err(Error::from_str("value below min_value"));
        //     }
        // }

        let old_val = self.value.clone();
        if let Some(ref on_update) = self.on_update {
            on_update(&old_val, &val);
        }

        if self.event_notifications == Some(true) {
            if let Some(ref event_emitter) = self.event_emitter {
                event_emitter
                    .lock()
                    .await
                    .emit(&Event::CharacteristicValueChanged {
                        aid: self.accessory_id,
                        iid: self.id,
                        value: json!(&val),
                    })
                    .await;
            }
        }

        self.value = val;

        Ok(())
    }

    /// Returns the `Unit` of a Characteristic.
    pub fn get_unit(&self) -> Option<Unit> { self.unit }

    /// Returns the maximum value of a Characteristic.
    pub fn get_max_value(&self) -> Option<T> { self.max_value.clone() }

    /// Sets the maximum value of a Characteristic.
    pub fn set_max_value(&mut self, val: Option<T>) { self.max_value = val; }

    /// Returns the minimum value of a Characteristic.
    pub fn get_min_value(&self) -> Option<T> { self.min_value.clone() }

    /// Sets the minimum value of a Characteristic.
    pub fn set_min_value(&mut self, val: Option<T>) { self.min_value = val; }

    /// Returns the step value of a Characteristic.
    pub fn get_step_value(&self) -> Option<T> { self.step_value.clone() }

    /// Returns the step value of a Characteristic.
    pub fn set_step_value(&mut self, val: Option<T>) { self.step_value = val; }

    /// Returns the maximum length of a Characteristic.
    pub fn get_max_len(&self) -> Option<u16> { self.max_len }

    /// Sets a callback on the Characteristic that is called every time a Controller attempts to read the value of a
    /// `Characteristic`. Returning a `Some(T)` from this function changes the value of the `Characteristic` before
    /// the Controller reads it so the Controller reads the new value.
    pub fn on_read(&mut self, f: impl Fn() -> Option<T> + 'static + Send + Sync) { self.on_read = Some(Box::new(f)); }

    /// Sets a callback on the Characteristic that is called every time a Controller attempts to update the value of a
    /// `Characteristic`. `old_val` is a reference to the current value of the `Characteristic` and `new_val` is a
    /// reference to the value the Controller attempts to change the `Characteristic`'s to.
    pub fn on_update(&mut self, f: impl Fn(&T, &T) + 'static + Send + Sync) { self.on_update = Some(Box::new(f)); }

    /// Sets a `hap::event::pointer::EventEmitter` on the Characteristic.
    pub(crate) fn set_event_emitter(&mut self, event_emitter: Option<pointer::EventEmitter>) {
        self.event_emitter = event_emitter;
    }
}

impl<T: Default + Clone + Serialize + Send + Sync> Serialize for Characteristic<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
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

/// Permission of a `Characteristic`.
#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
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

/// Unit of a `Characteristic`.
#[derive(Debug, Copy, Clone, Serialize)]
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

/// Format (data type) of a `Characteristic`.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
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
    #[serde(rename = "uint64")]
    UInt64,
    #[serde(rename = "int32")]
    Int32,
    #[serde(rename = "tlv8")]
    Tlv8,
    #[serde(rename = "data")]
    Data,
}

impl Default for Format {
    fn default() -> Format { Format::String }
}

/// `HapCharacteristic` is implemented by every `Characteristic`.
#[async_trait]
pub trait HapCharacteristic: HapCharacteristicSetup + erased_serde::Serialize + Send + Sync {
    /// Returns the ID of a Characteristic.
    fn get_id(&self) -> u64;
    /// Returns the `HapType` of a Characteristic.
    fn get_type(&self) -> HapType;
    /// Returns the `Format` of a Characteristic.
    fn get_format(&self) -> Format;
    /// Returns the `Perm`s of a Characteristic.
    fn get_perms(&self) -> Vec<Perm>;
    /// Returns the event notifications value of a Characteristic.
    fn get_event_notifications(&self) -> Option<bool>;
    /// Sets the event notifications value of a Characteristic.
    fn set_event_notifications(&mut self, event_notifications: Option<bool>);
    /// Returns the value of a Characteristic.
    async fn get_value(&mut self) -> Result<serde_json::Value>;
    /// Sets the value of a Characteristic.
    async fn set_value(&mut self, value: serde_json::Value) -> Result<()>;
    /// Returns the `Unit` of a Characteristic.
    fn get_unit(&self) -> Option<Unit>;
    /// Returns the maximum value of a Characteristic.
    fn get_max_value(&self) -> Option<serde_json::Value>;
    /// Returns the minimum value of a Characteristic.
    fn get_min_value(&self) -> Option<serde_json::Value>;
    /// Returns the step value of a Characteristic.
    fn get_step_value(&self) -> Option<serde_json::Value>;
    /// Returns the maximum length of a Characteristic.
    fn get_max_len(&self) -> Option<u16>;
}

serialize_trait_object!(HapCharacteristic);

pub trait HapCharacteristicSetup {
    /// Sets a `hap::event::pointer::EventEmitter` on the Characteristic.
    fn set_event_emitter(&mut self, event_emitter: Option<pointer::EventEmitter>);
}

pub trait OnReadFn<T: Default + Clone + Serialize + Send + Sync>: Fn() -> Option<T> + 'static + Send + Sync {}
impl<F, T: Default + Clone + Serialize + Send + Sync> OnReadFn<T> for F where
    F: Fn() -> Option<T> + 'static + Send + Sync
{
}

impl<T: Default + Clone + Serialize + Send + Sync> fmt::Debug for dyn OnReadFn<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "OnReadFn") }
}

pub trait OnUpdateFn<T: Default + Clone + Serialize + Send + Sync>: Fn(&T, &T) + 'static + Send + Sync {}
impl<F, T: Default + Clone + Serialize + Send + Sync> OnUpdateFn<T> for F where F: Fn(&T, &T) + 'static + Send + Sync {}

impl<T: Default + Clone + Serialize + Send + Sync> fmt::Debug for dyn OnUpdateFn<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "OnUpdateFn") }
}

pub trait CharacteristicCallbacks<T: Default + Clone + Serialize + Send + Sync> {
    /// Sets a callback on the Characteristic that is called every time a Controller attempts to read the value of a
    /// `Characteristic`. Returning a `Some(T)` from this function changes the value of the `Characteristic` before
    /// the Controller reads it so the Controller reads the new value.
    fn on_read(&mut self, f: impl OnReadFn<T>);
    /// Sets a callback on the Characteristic that is called every time a Controller attempts to update the value of a
    /// `Characteristic`. `old_val` is a reference to the current value of the `Characteristic` and `new_val` is a
    /// reference to the value the Controller attempts to change the `Characteristic`'s to.
    fn on_update(&mut self, f: impl OnUpdateFn<T>);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_serialization() {
        let characteristic = Characteristic::<u16> {
            id: 1,
            accessory_id: 1,
            hap_type: HapType::CurrentTiltAngle,
            format: Format::UInt16,
            perms: vec![Perm::PairedRead, Perm::Events],
            description: Some("Acme Tilt Angle".into()),
            event_notifications: Some(true),

            value: 123,
            unit: Some(Unit::ArcDegrees),

            max_value: Some(360),
            min_value: Some(0),
            step_value: Some(1),
            max_len: None,
            max_data_len: None,
            valid_values: None,
            valid_values_range: Some([0, 360]),

            on_read: None,
            on_update: None,

            event_emitter: None,
        };
        let json = serde_json::to_string(&characteristic).unwrap();
        assert_eq!(json, "{\"iid\":1,\"type\":\"C1\",\"format\":\"uint16\",\"perms\":[\"pr\",\"ev\"],\"description\":\"Acme Tilt Angle\",\"ev\":true,\"value\":123,\"unit\":\"arcdegrees\",\"maxValue\":360,\"minValue\":0,\"minStep\":1,\"valid-values-range\":[0,360]}".to_string());
    }
}
