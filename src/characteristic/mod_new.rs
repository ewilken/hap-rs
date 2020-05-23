use std::{ops::Deref, sync::Arc};

use erased_serde::serialize_trait_object;
use futures::{
    future::{BoxFuture, FutureExt},
    lock::Mutex,
};
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
pub struct Inner<T: Default + Clone + Serialize + Send + Sync> {
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

    readable: Option<Box<dyn Readable<T> + Send + Sync>>,
    updatable: Option<Box<dyn Updatable<T> + Send + Sync>>,

    event_emitter: Option<pointer::EventEmitter>,
}

/// A Characteristic. A characteristic is a feature that represents data or an associated behavior
/// of a service. The characteristic is defined by a universally unique type, and has additional
/// properties that determine how the value of the characteristic can be accessed.
#[derive(Clone, Default)]
pub struct Characteristic<T: Default + Clone + Serialize + Send + Sync> {
    pub inner: Arc<Mutex<Inner<T>>>,
}

impl<T: Default + Clone + Serialize + Send + Sync> Characteristic<T>
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
    pub fn get_id(&self) -> BoxFuture<u64> { async { self.inner.lock().await.id }.boxed() }

    /// Sets the ID of a Characteristic.
    pub fn set_id(&mut self, id: u64) -> BoxFuture<()> {
        async {
            self.inner.lock().await.id = id;
        }
        .boxed()
    }

    /// Sets the Accessory ID of a Characteristic.
    pub fn set_accessory_id(&mut self, accessory_id: u64) -> BoxFuture<()> {
        async {
            self.inner.lock().await.accessory_id = accessory_id;
        }
        .boxed()
    }

    /// Returns the `HapType` of a Characteristic.
    pub fn get_type(&self) -> BoxFuture<HapType> { async { self.inner.lock().await.hap_type }.boxed() }

    /// Returns the `Format` of a Characteristic.
    pub fn get_format(&self) -> BoxFuture<Format> { async { self.inner.lock().await.format }.boxed() }

    /// Returns the `Perm`s of a Characteristic.
    pub fn get_perms(&self) -> BoxFuture<Vec<Perm>> { async { self.inner.lock().await.perms.clone() }.boxed() }

    /// Sets the description of a Characteristic.
    pub fn set_description(&mut self, description: Option<String>) -> BoxFuture<()> {
        async {
            self.inner.lock().await.description = description;
        }
        .boxed()
    }

    /// Returns the event notifications value of a Characteristic.
    pub fn get_event_notifications(&self) -> BoxFuture<Option<bool>> {
        async { self.inner.lock().await.event_notifications }.boxed()
    }

    /// Sets the event notifications value of a Characteristic.
    pub fn set_event_notifications(&mut self, event_notifications: Option<bool>) -> BoxFuture<()> {
        async {
            self.inner.lock().await.event_notifications = event_notifications;
        }
        .boxed()
    }

    /// Returns the value of a Characteristic.
    pub fn get_value(&mut self) -> BoxFuture<Result<T>> {
        async {
            let mut val = None;
            {
                let mut inner = self.inner.lock().await;
                let hap_type = inner.hap_type;
                if let Some(ref mut readable) = inner.readable {
                    val = readable.on_read(hap_type);
                }
            }
            if let Some(v) = val {
                self.set_value(v).await?;
            }

            Ok(self.inner.lock().await.value.clone())
        }
        .boxed()
    }

    /// Sets the value of a Characteristic.
    pub fn set_value(&mut self, val: T) -> BoxFuture<Result<()>> {
        async move {
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
                let mut inner = self.inner.lock().await;
                let old_val = inner.value.clone();
                let hap_type = inner.hap_type;
                if let Some(ref mut updatable) = inner.updatable {
                    updatable.on_update(&old_val, &val, hap_type);
                }
            }

            {
                let inner = self.inner.lock().await;
                if inner.event_notifications == Some(true) {
                    if let Some(ref event_emitter) = inner.event_emitter {
                        event_emitter.lock().await.emit(&Event::CharacteristicValueChanged {
                            aid: inner.accessory_id,
                            iid: inner.id,
                            value: json!(&val),
                        });
                    }
                }
            }

            let mut inner = self.inner.lock().await;
            inner.value = val;
            drop(inner);

            Ok(())
        }
        .boxed()
    }

    /// Returns the `Unit` of a Characteristic.
    pub fn get_unit(&self) -> BoxFuture<Option<Unit>> { async { self.inner.lock().await.unit }.boxed() }

    /// Returns the maximum value of a Characteristic.
    pub fn get_max_value(&self) -> BoxFuture<Option<T>> { async { self.inner.lock().await.max_value.clone() }.boxed() }

    /// Sets the maximum value of a Characteristic.
    pub fn set_max_value(&mut self, val: Option<T>) -> BoxFuture<()> {
        async {
            self.inner.lock().await.max_value = val;
        }
        .boxed()
    }

    /// Returns the minimum value of a Characteristic.
    pub fn get_min_value(&self) -> BoxFuture<Option<T>> { async { self.inner.lock().await.min_value.clone() }.boxed() }

    /// Sets the minimum value of a Characteristic.
    pub fn set_min_value(&mut self, val: Option<T>) -> BoxFuture<()> {
        async {
            self.inner.lock().await.min_value = val;
        }
        .boxed()
    }

    /// Returns the step value of a Characteristic.
    pub fn get_step_value(&self) -> BoxFuture<Option<T>> {
        async { self.inner.lock().await.step_value.clone() }.boxed()
    }

    /// Returns the step value of a Characteristic.
    pub fn set_step_value(&mut self, val: Option<T>) -> BoxFuture<()> {
        async {
            self.inner.lock().await.step_value = val;
        }
        .boxed()
    }

    /// Returns the maximum length of a Characteristic.
    pub fn get_max_len(&self) -> BoxFuture<Option<u16>> { async { self.inner.lock().await.max_len }.boxed() }

    /// Sets a `Readable` on the Characteristic.
    pub fn set_readable(&mut self, readable: impl Readable<T> + 'static + Send + Sync) -> BoxFuture<()> {
        async {
            self.inner.lock().await.readable = Some(Box::new(readable));
        }
        .boxed()
    }

    /// Sets an `Readable` on the Characteristic.
    pub fn set_updatable(&mut self, updatable: impl Updatable<T> + 'static + Send + Sync) -> BoxFuture<()> {
        async {
            self.inner.lock().await.updatable = Some(Box::new(updatable));
        }
        .boxed()
    }

    /// Sets a `hap::event::pointer::EventEmitter` on the Characteristic.
    pub fn set_event_emitter(&mut self, event_emitter: Option<pointer::EventEmitter>) -> BoxFuture<()> {
        async {
            self.inner.lock().await.event_emitter = event_emitter;
        }
        .boxed()
    }

    /// Consumes self into a serializable helper type.
    async fn to_serializable(self) -> SerializableCharacteristic<T> {
        let inner = self.inner.lock().await.deref();
        SerializableCharacteristic { inner: *inner }
    }
}

/// Helper type for async serialization.
#[derive(Debug)]
pub struct SerializableCharacteristic<T: Default + Clone + Serialize + Send + Sync> {
    pub inner: Inner<T>,
}

impl<T: Default + Clone + Serialize + Send + Sync> Serialize for SerializableCharacteristic<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Characteristic", 15)?;
        let inner = self.inner;
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
    fn get_id(&self) -> BoxFuture<u64>;
    /// Sets the ID of a Characteristic.
    fn set_id(&mut self, id: u64) -> BoxFuture<()>;
    /// Sets the Accessory ID of a Characteristic.
    fn set_accessory_id(&mut self, accessory_id: u64) -> BoxFuture<()>;
    /// Returns the `HapType` of a Characteristic.
    fn get_type(&self) -> BoxFuture<HapType>;
    /// Returns the `Format` of a Characteristic.
    fn get_format(&self) -> BoxFuture<Format>;
    /// Returns the `Perm`s of a Characteristic.
    fn get_perms(&self) -> BoxFuture<Vec<Perm>>;
    /// Returns the event notifications value of a Characteristic.
    fn get_event_notifications(&self) -> BoxFuture<Option<bool>>;
    /// Sets the event notifications value of a Characteristic.
    fn set_event_notifications(&mut self, event_notifications: Option<bool>) -> BoxFuture<()>;
    /// Returns the value of a Characteristic.
    fn get_value(&mut self) -> BoxFuture<serde_json::Value>;
    /// Sets the value of a Characteristic.
    fn set_value(&mut self, value: serde_json::Value) -> BoxFuture<Result<()>>;
    /// Returns the `Unit` of a Characteristic.
    fn get_unit(&self) -> BoxFuture<Option<Unit>>;
    /// Returns the maximum value of a Characteristic.
    fn get_max_value(&self) -> BoxFuture<Option<serde_json::Value>>;
    /// Returns the minimum value of a Characteristic.
    fn get_min_value(&self) -> BoxFuture<Option<serde_json::Value>>;
    /// Returns the step value of a Characteristic.
    fn get_step_value(&self) -> BoxFuture<Option<serde_json::Value>>;
    /// Returns the maximum length of a Characteristic.
    fn get_max_len(&self) -> BoxFuture<Option<u16>>;
    /// Sets a `hap::event::pointer::EventEmitter` on the Characteristic.
    fn set_event_emitter(&mut self, event_emitter: Option<pointer::EventEmitter>) -> BoxFuture<()>;
}

serialize_trait_object!(HapCharacteristic);

impl<T: Default + Clone + Serialize + Send + Sync> HapCharacteristic for Characteristic<T>
where
    for<'de> T: Deserialize<'de>,
{
    fn get_id(&self) -> BoxFuture<u64> { self.get_id() }

    fn set_id(&mut self, id: u64) -> BoxFuture<()> { self.set_id(id) }

    fn set_accessory_id(&mut self, accessory_id: u64) -> BoxFuture<()> { self.set_accessory_id(accessory_id) }

    fn get_type(&self) -> BoxFuture<HapType> { self.get_type() }

    fn get_format(&self) -> BoxFuture<Format> { self.get_format() }

    fn get_perms(&self) -> BoxFuture<Vec<Perm>> { self.get_perms() }

    fn get_event_notifications(&self) -> BoxFuture<Option<bool>> { self.get_event_notifications() }

    fn set_event_notifications(&mut self, event_notifications: Option<bool>) -> BoxFuture<()> {
        self.set_event_notifications(event_notifications)
    }

    fn get_value(&mut self) -> BoxFuture<serde_json::Value> { Ok(json!(self.get_value()?)) }

    fn set_value(&mut self, value: serde_json::Value) -> BoxFuture<Result<()>> {
        async {
            let v;
            // for whatever reason, the controller is setting boolean values either as a boolean or as an integer
            if self.inner.lock().await.format == Format::Bool && value.is_number() {
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
            self.set_value(v).await
        }
        .boxed()
    }

    fn get_unit(&self) -> BoxFuture<Option<Unit>> { self.get_unit() }

    fn get_max_value(&self) -> BoxFuture<Option<serde_json::Value>> {
        Ok(match self.get_max_value()? {
            Some(v) => Some(json!(v)),
            None => None,
        })
    }

    fn get_min_value(&self) -> BoxFuture<Option<serde_json::Value>> {
        Ok(match self.get_min_value()? {
            Some(v) => Some(json!(v)),
            None => None,
        })
    }

    fn get_step_value(&self) -> BoxFuture<Option<serde_json::Value>> {
        Ok(match self.get_step_value()? {
            Some(v) => Some(json!(v)),
            None => None,
        })
    }

    fn get_max_len(&self) -> BoxFuture<Option<u16>> { self.get_max_len() }

    fn set_event_emitter(&mut self, event_emitter: Option<pointer::EventEmitter>) -> BoxFuture<()> {
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
