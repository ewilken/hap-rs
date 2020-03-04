use std::sync::{Arc, Mutex};

use erased_serde::{self, serialize_trait_object};
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize,
    Serialize,
};
use serde_json::json;

use crate::{event::Event, pointer, Error, HapType, Result};

mod generated;

pub use generated::*;

/// Inner type of a `Characteristic`.
#[derive(Default)]
pub struct Inner<T: Default + Clone + Serialize> {
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

    readable: Option<Box<dyn Readable<T> + Send>>,
    updatable: Option<Box<dyn Updatable<T> + Send>>,

    event_emitter: Option<pointer::EventEmitter>,
}

/// A Characteristic. A characteristic is a feature that represents data or an associated behavior
/// of a service. The characteristic is defined by a universally unique type, and has additional
/// properties that determine how the value of the characteristic can be accessed.
#[derive(Clone, Default)]
pub struct Characteristic<T: Default + Clone + Serialize> {
    pub inner: Arc<Mutex<Inner<T>>>,
}

impl<T: Default + Clone + Serialize> Characteristic<T>
where
    for<'de> T: Deserialize<'de>,
{
    /// Creates a new `Characteristic`.
    pub fn new(inner: Inner<T>) -> Characteristic<T> {
        Characteristic {
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    /// Returns the ID of a Characteristic.
    pub fn get_id(&self) -> Result<u64> { Ok(self.inner.lock().expect("couldn't access characteristic").id) }

    /// Sets the ID of a Characteristic.
    pub fn set_id(&mut self, id: u64) -> Result<()> {
        self.inner.lock().expect("couldn't access characteristic").id = id;
        Ok(())
    }

    /// Sets the Accessory ID of a Characteristic.
    pub fn set_accessory_id(&mut self, accessory_id: u64) -> Result<()> {
        self.inner.lock().expect("couldn't access characteristic").accessory_id = accessory_id;
        Ok(())
    }

    /// Returns the `HapType` of a Characteristic.
    pub fn get_type(&self) -> Result<HapType> {
        Ok(self.inner.lock().expect("couldn't access characteristic").hap_type)
    }

    /// Returns the `Format` of a Characteristic.
    pub fn get_format(&self) -> Result<Format> { Ok(self.inner.lock().expect("couldn't access characteristic").format) }

    /// Returns the `Perm`s of a Characteristic.
    pub fn get_perms(&self) -> Result<Vec<Perm>> {
        Ok(self.inner.lock().expect("couldn't access characteristic").perms.clone())
    }

    /// Sets the description of a Characteristic.
    pub fn set_description(&mut self, description: Option<String>) -> Result<()> {
        self.inner.lock().expect("couldn't access characteristic").description = description;
        Ok(())
    }

    /// Returns the event notifications value of a Characteristic.
    pub fn get_event_notifications(&self) -> Result<Option<bool>> {
        Ok(self
            .inner
            .lock()
            .expect("couldn't access characteristic")
            .event_notifications)
    }

    /// Sets the event notifications value of a Characteristic.
    pub fn set_event_notifications(&mut self, event_notifications: Option<bool>) -> Result<()> {
        self.inner
            .lock()
            .expect("couldn't access characteristic")
            .event_notifications = event_notifications;
        Ok(())
    }

    /// Returns the value of a Characteristic.
    pub fn get_value(&mut self) -> Result<T> {
        let mut val = None;
        {
            let mut inner = self.inner.lock().expect("couldn't access characteristic");
            let hap_type = inner.hap_type;
            if let Some(ref mut readable) = inner.readable {
                val = readable.on_read(hap_type);
            }
        }
        if let Some(v) = val {
            self.set_value(v)?;
        }

        Ok(self.inner.lock().expect("couldn't access characteristic").value.clone())
    }

    /// Sets the value of a Characteristic.
    pub fn set_value(&mut self, val: T) -> Result<()> {
        // TODO - check for min/max on types implementing PartialOrd
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

        {
            let mut inner = self.inner.lock().expect("couldn't access characteristic");
            let old_val = inner.value.clone();
            let hap_type = inner.hap_type;
            if let Some(ref mut updatable) = inner.updatable {
                updatable.on_update(&old_val, &val, hap_type);
            }
        }

        {
            let inner = self.inner.lock().expect("couldn't access characteristic");
            if inner.event_notifications == Some(true) {
                if let Some(ref event_emitter) = inner.event_emitter {
                    event_emitter.lock().expect("couldn't access event_emitter").emit(
                        &Event::CharacteristicValueChanged {
                            aid: inner.accessory_id,
                            iid: inner.id,
                            value: json!(&val),
                        },
                    );
                }
            }
        }

        self.inner.lock().expect("couldn't access characteristic").value = val;

        Ok(())
    }

    /// Returns the `Unit` of a Characteristic.
    pub fn get_unit(&self) -> Result<Option<Unit>> {
        Ok(self.inner.lock().expect("couldn't access characteristic").unit)
    }

    /// Returns the maximum value of a Characteristic.
    pub fn get_max_value(&self) -> Result<Option<T>> {
        Ok(self
            .inner
            .lock()
            .expect("couldn't access characteristic")
            .max_value
            .clone())
    }

    /// Sets the maximum value of a Characteristic.
    pub fn set_max_value(&mut self, val: Option<T>) -> Result<()> {
        self.inner.lock().expect("couldn't access characteristic").max_value = val;
        Ok(())
    }

    /// Returns the minimum value of a Characteristic.
    pub fn get_min_value(&self) -> Result<Option<T>> {
        Ok(self
            .inner
            .lock()
            .expect("couldn't access characteristic")
            .min_value
            .clone())
    }

    /// Sets the minimum value of a Characteristic.
    pub fn set_min_value(&mut self, val: Option<T>) -> Result<()> {
        self.inner.lock().expect("couldn't access characteristic").min_value = val;
        Ok(())
    }

    /// Returns the step value of a Characteristic.
    pub fn get_step_value(&self) -> Result<Option<T>> {
        Ok(self
            .inner
            .lock()
            .expect("couldn't access characteristic")
            .step_value
            .clone())
    }

    /// Returns the step value of a Characteristic.
    pub fn set_step_value(&mut self, val: Option<T>) -> Result<()> {
        self.inner.lock().expect("couldn't access characteristic").step_value = val;
        Ok(())
    }

    /// Returns the maximum length of a Characteristic.
    pub fn get_max_len(&self) -> Result<Option<u16>> {
        Ok(self.inner.lock().expect("couldn't access characteristic").max_len)
    }

    /// Sets a `Readable` on the Characteristic.
    pub fn set_readable(&mut self, readable: impl Readable<T> + 'static + Send) -> Result<()> {
        self.inner.lock().expect("couldn't access characteristic").readable = Some(Box::new(readable));
        Ok(())
    }

    /// Sets an `Readable` on the Characteristic.
    pub fn set_updatable(&mut self, updatable: impl Updatable<T> + 'static + Send) -> Result<()> {
        self.inner.lock().expect("couldn't access characteristic").updatable = Some(Box::new(updatable));
        Ok(())
    }

    /// Sets a `hap::event::pointer::EventEmitter` on the Characteristic.
    pub fn set_event_emitter(&mut self, event_emitter: Option<pointer::EventEmitter>) -> Result<()> {
        self.inner.lock().expect("couldn't access characteristic").event_emitter = event_emitter;
        Ok(())
    }
}

impl<T: Default + Clone + Serialize> Serialize for Characteristic<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Characteristic", 15)?;
        let inner = self.inner.lock().expect("couldn't access characteristic");
        state.serialize_field("iid", &inner.id)?;
        state.serialize_field("type", &inner.hap_type)?;
        state.serialize_field("format", &inner.format)?;
        state.serialize_field("perms", &inner.perms)?;
        if let Some(ref description) = inner.description {
            state.serialize_field("description", description)?;
        }
        if let Some(ref event_notifications) = inner.event_notifications {
            state.serialize_field("ev", event_notifications)?;
        }

        if inner.perms.contains(&Perm::PairedRead) {
            state.serialize_field("value", &inner.value)?;
        }
        if let Some(ref unit) = inner.unit {
            state.serialize_field("unit", unit)?;
        }
        if let Some(ref max_value) = inner.max_value {
            state.serialize_field("maxValue", max_value)?;
        }
        if let Some(ref min_value) = inner.min_value {
            state.serialize_field("minValue", min_value)?;
        }
        if let Some(ref step_value) = inner.step_value {
            state.serialize_field("minStep", step_value)?;
        }
        if let Some(ref max_len) = inner.max_len {
            state.serialize_field("maxLen", max_len)?;
        }
        if let Some(ref max_data_len) = inner.max_data_len {
            state.serialize_field("maxDataLen", max_data_len)?;
        }
        if let Some(ref valid_values) = inner.valid_values {
            state.serialize_field("valid-values", valid_values)?;
        }
        if let Some(ref valid_values_range) = inner.valid_values_range {
            state.serialize_field("valid-values-range", valid_values_range)?;
        }
        state.end()
    }
}

/// `HapCharacteristic` is implemented by the inner type of every `Characteristic`.
pub trait HapCharacteristic: erased_serde::Serialize {
    /// Returns the ID of a Characteristic.
    fn get_id(&self) -> Result<u64>;
    /// Sets the ID of a Characteristic.
    fn set_id(&mut self, id: u64) -> Result<()>;
    /// Sets the Accessory ID of a Characteristic.
    fn set_accessory_id(&mut self, accessory_id: u64) -> Result<()>;
    /// Returns the `HapType` of a Characteristic.
    fn get_type(&self) -> Result<HapType>;
    /// Returns the `Format` of a Characteristic.
    fn get_format(&self) -> Result<Format>;
    /// Returns the `Perm`s of a Characteristic.
    fn get_perms(&self) -> Result<Vec<Perm>>;
    /// Returns the event notifications value of a Characteristic.
    fn get_event_notifications(&self) -> Result<Option<bool>>;
    /// Sets the event notifications value of a Characteristic.
    fn set_event_notifications(&mut self, event_notifications: Option<bool>) -> Result<()>;
    /// Returns the value of a Characteristic.
    fn get_value(&mut self) -> Result<serde_json::Value>;
    /// Sets the value of a Characteristic.
    fn set_value(&mut self, value: serde_json::Value) -> Result<()>;
    /// Returns the `Unit` of a Characteristic.
    fn get_unit(&self) -> Result<Option<Unit>>;
    /// Returns the maximum value of a Characteristic.
    fn get_max_value(&self) -> Result<Option<serde_json::Value>>;
    /// Returns the minimum value of a Characteristic.
    fn get_min_value(&self) -> Result<Option<serde_json::Value>>;
    /// Returns the step value of a Characteristic.
    fn get_step_value(&self) -> Result<Option<serde_json::Value>>;
    /// Returns the maximum length of a Characteristic.
    fn get_max_len(&self) -> Result<Option<u16>>;
    /// Sets a `hap::event::pointer::EventEmitter` on the Characteristic.
    fn set_event_emitter(&mut self, event_emitter: Option<pointer::EventEmitter>) -> Result<()>;
}

serialize_trait_object!(HapCharacteristic);

impl<T: Default + Clone + Serialize> HapCharacteristic for Characteristic<T>
where
    for<'de> T: Deserialize<'de>,
{
    fn get_id(&self) -> Result<u64> { self.get_id() }

    fn set_id(&mut self, id: u64) -> Result<()> { self.set_id(id) }

    fn set_accessory_id(&mut self, accessory_id: u64) -> Result<()> { self.set_accessory_id(accessory_id) }

    fn get_type(&self) -> Result<HapType> { self.get_type() }

    fn get_format(&self) -> Result<Format> { self.get_format() }

    fn get_perms(&self) -> Result<Vec<Perm>> { self.get_perms() }

    fn get_event_notifications(&self) -> Result<Option<bool>> { self.get_event_notifications() }

    fn set_event_notifications(&mut self, event_notifications: Option<bool>) -> Result<()> {
        self.set_event_notifications(event_notifications)
    }

    fn get_value(&mut self) -> Result<serde_json::Value> { Ok(json!(self.get_value()?)) }

    fn set_value(&mut self, value: serde_json::Value) -> Result<()> {
        let v;
        // for whatever reason, the controller is setting boolean values either as a boolean or as an integer
        if self.inner.lock().expect("couldn't access characteristic").format == Format::Bool && value.is_number() {
            let num_v: u8 = serde_json::from_value(value)?;
            if num_v == 0 {
                v = serde_json::from_value(json!(false))?;
            } else if num_v == 1 {
                v = serde_json::from_value(json!(true))?;
            } else {
                return Err(Error::from_str("invalid value for bool characteristic"));
            }
        } else {
            v = serde_json::from_value(value)?;
        }
        self.set_value(v)
    }

    fn get_unit(&self) -> Result<Option<Unit>> { self.get_unit() }

    fn get_max_value(&self) -> Result<Option<serde_json::Value>> {
        Ok(match self.get_max_value()? {
            Some(v) => Some(json!(v)),
            None => None,
        })
    }

    fn get_min_value(&self) -> Result<Option<serde_json::Value>> {
        Ok(match self.get_min_value()? {
            Some(v) => Some(json!(v)),
            None => None,
        })
    }

    fn get_step_value(&self) -> Result<Option<serde_json::Value>> {
        Ok(match self.get_step_value()? {
            Some(v) => Some(json!(v)),
            None => None,
        })
    }

    fn get_max_len(&self) -> Result<Option<u16>> { self.get_max_len() }

    fn set_event_emitter(&mut self, event_emitter: Option<pointer::EventEmitter>) -> Result<()> {
        self.set_event_emitter(event_emitter)
    }
}

/// `Readable` can be implemented to react to the remote read of a `Characteristic`.
pub trait Readable<T: Default + Serialize> {
    /// This function is called every time a Controller attempts to read the value of a
    /// `Characteristic`. Returning a `Some(T)` from this function changes the value of the
    /// `Characteristic` before the Controller reads it so the Controller reads the new value.
    fn on_read(&mut self, hap_type: HapType) -> Option<T>;
}

/// `Updatable` can be implemented to react to the remote update of a `Characteristic`.
pub trait Updatable<T: Default + Serialize> {
    /// This function is called every time a Controller attempts to update the value of a
    /// `Characteristic`. `old_val` is a reference to the current value of the `Characteristic` and
    /// `new_val` is a reference to the value the Controller attempts to change the
    /// `Characteristic`'s to.
    fn on_update(&mut self, old_val: &T, new_val: &T, hap_type: HapType);
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
