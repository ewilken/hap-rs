use async_trait::async_trait;
use erased_serde::serialize_trait_object;
use futures::future::BoxFuture;
use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize,
    Serialize,
};
use serde_json::json;
use std::fmt;

use crate::{event::Event, pointer, Error, HapType, Result};

mod generated;

pub use generated::*;

/// A characteristic. A characteristic is a feature that represents data or an associated behavior of a service. The
/// characteristic is defined by a universally unique type, and has additional properties that determine how the value
/// of the characteristic can be accessed.
#[derive(Default)]
pub struct Characteristic<T: fmt::Debug + Default + Clone + Serialize + Send + Sync> {
    /// Instance ID; integer assigned by the server to uniquely identify the HAP characteristic object.
    id: u64,
    /// ID of the accessory the characteristic belongs to.
    accessory_id: u64,
    /// The type of the characteristic.
    hap_type: HapType,
    /// Format of the value.
    format: Format,
    /// Permissions describing the capabilities of the characteristic.
    perms: Vec<Perm>,
    /// String describing the characteristic on a manufacturer-specific basis, such as an indoor versus outdoor
    /// temperature reading.
    description: Option<String>,
    /// Boolean indicating if event notifications are enabled for this characteristic.
    event_notifications: Option<bool>,

    /// The value of the characteristic, which must conform to the `format` property. This property must be present if
    /// and only if the characteristic contains the Paired Read permission.
    value: T,
    /// Unit of the value, e.g. `Celsius`.
    unit: Option<Unit>,

    /// Maximum value for the characteristic, which is only appropriate for characteristics that have a format of `Int`
    /// or `Float`.
    max_value: Option<T>, // TODO - use this value in `set_value`
    // Minimum value for the characteristic, which is only appropriate for characteristics that have a format of `Int`
    // or `Float`.
    min_value: Option<T>, // TODO - use this value in `set_value`
    /// Minimum step value for the characteristic, which is only appropriate for characteristics that have a format of
    /// ”int” or ”float”. For example, if this were 0.15, the characteris- tic value can be incremented from the min-
    /// imum value in multiples of 0.15. For “float”, the “Value” needs to be rounded on the ac- cessory side to the
    /// closest allowed value per the ”Step Value” (e.g. a value of 10.150001 received on the accessory side with a
    /// ”Step Value” of 0.15 and a ”Minimum Value” of 10.0 needs to be interpreted as 10.15).
    step_value: Option<T>, // TODO - use this value in `set_value`
    /// Maximum number of characters if the for- mat is ”string”. If this property is omitted for ”string” formats,
    /// then the default value is 64. The maximum value allowed is 256.
    max_len: Option<u16>, // TODO - use this value in `set_value`
    /// Maximum number of characters if the format is ”data”. If this property is omitted for ”data” formats, then the
    /// default value is 2097152.
    max_data_len: Option<u32>, // TODO - use this value in `set_value`
    /// An array of numbers where each element represents a valid value.
    valid_values: Option<Vec<T>>, // TODO - use this value in `set_value`
    /// A 2 element array representing the starting value and ending value of the range of valid values.
    valid_values_range: Option<[T; 2]>, // TODO - use this value in `set_value`

    /// Specified TTL in milliseconds the controller requests the accessory to securely execute a write command.
    /// Maximum value of this is 9007199254740991.
    ttl: Option<u64>, // TODO - use this value in `set_value`
    /// 64-bit unsigned integer assigned by the controller to uniquely identify the timed write transaction.
    pid: Option<u64>, // TODO - use this value in `set_value`

    /// Sets a callback function on a characteristic that is called every time a controller attempts to read its value.
    /// Returning a `Some(T)` from this function changes the value of the characteristic before the controller reads
    /// it so the Controller reads the new value.
    on_read: Option<Box<dyn OnReadFn<T>>>,
    /// Sets a callback function on a characteristic that is called every time a controller attempts to update its
    /// value. The first argument is a reference to the current value of the characteristic and the second argument is
    /// a reference to the value the controller attempts to change the characteristic's to.
    on_update: Option<Box<dyn OnUpdateFn<T>>>,
    /// Sets an async callback function on a characteristic that is driven to completion by the async runtime driving
    /// the HAP server every time a controller attempts to read its value. Returning a `Some(T)` from this function
    /// changes the value of the characteristic before the controller reads it so the controller reads the new value.
    on_read_async: Option<Box<dyn OnReadFuture<T>>>,
    /// Sets an async callback function on a characteristic that is driven to completion by the async runtime driving
    /// the HAP server every time a controller attempts to update its value. The first argument is a reference to the
    /// current value of the characteristic and the second argument is a reference to the value the controller attempts
    /// to change the characteristic's to.
    on_update_async: Option<Box<dyn OnUpdateFuture<T>>>,

    event_emitter: Option<pointer::EventEmitter>,
}

impl<T: fmt::Debug + Default + Clone + Serialize + Send + Sync> fmt::Debug for Characteristic<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Characteristic")
            .field("id", &self.id)
            .field("accessory_id", &self.accessory_id)
            .field("hap_type", &self.hap_type)
            .field("format", &self.format)
            .field("perms", &self.perms)
            .field("description", &self.description)
            .field("event_notifications", &self.event_notifications)
            .field("value", &self.value)
            .field("unit", &self.unit)
            .field("max_value", &self.max_value)
            .field("min_value", &self.min_value)
            .field("step_value", &self.step_value)
            .field("max_len", &self.max_len)
            .field("max_data_len", &self.max_data_len)
            .field("valid_values", &self.valid_values)
            .field("valid_values_range", &self.valid_values_range)
            .field("ttl", &self.ttl)
            .field("pid", &self.pid)
            .finish()
    }
}

impl<T: fmt::Debug + Default + Clone + Serialize + Send + Sync> Characteristic<T>
where
    for<'de> T: Deserialize<'de>,
{
    /// Creates a new characteristic.
    pub fn new(
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
        ttl: Option<u64>,
        pid: Option<u64>,
    ) -> Self {
        Self {
            id,
            accessory_id,
            hap_type,
            format,
            perms,
            description,
            event_notifications,
            value,
            unit,
            max_value,
            min_value,
            step_value,
            max_len,
            max_data_len,
            valid_values,
            valid_values_range,
            ttl,
            pid,
            on_read: None,
            on_update: None,
            on_read_async: None,
            on_update_async: None,
            event_emitter: None,
        }
    }

    /// Returns the ID of the characteristic.
    pub fn get_id(&self) -> u64 { self.id }

    /// Sets the ID of the characteristic.
    pub fn set_id(&mut self, id: u64) { self.id = id; }

    /// Returns the [`HapType`](HapType) of the characteristic.
    pub fn get_type(&self) -> HapType { self.hap_type }

    /// Sets the [`HapType`](HapType) of the characteristic.
    pub fn set_type(&mut self, hap_type: HapType) { self.hap_type = hap_type; }

    /// Returns the [`Format`](Format) of the characteristic.
    pub fn get_format(&self) -> Format { self.format }

    /// Sets the [`Format`](Format) of the characteristic.
    pub fn set_format(&mut self, format: Format) { self.format = format; }

    /// Returns the [`Perm`](Perm)s of the characteristic.
    pub fn get_perms(&self) -> Vec<Perm> { self.perms.clone() }

    /// Sets the [`Perm`](Perm)s of the characteristic.
    pub fn set_perms(&mut self, perms: Vec<Perm>) { self.perms = perms; }

    /// Returns the description of the characteristic.
    pub fn get_description(&self) -> Option<String> { self.description.clone() }

    /// Sets the description of the characteristic.
    pub fn set_description(&mut self, description: Option<String>) { self.description = description; }

    /// Returns the `event_notifications` value of the characteristic.
    pub fn get_event_notifications(&self) -> Option<bool> { self.event_notifications }

    /// Sets the `event_notifications` value of the characteristic.
    pub fn set_event_notifications(&mut self, event_notifications: Option<bool>) {
        self.event_notifications = event_notifications;
    }

    /// Returns the value of the characteristic.
    pub async fn get_value(&mut self) -> Result<T> {
        let mut val = None;
        if let Some(ref mut on_read) = self.on_read {
            val = on_read().map_err(|e| Error::ValueOnRead(e))?;
        }
        if let Some(ref mut on_read_async) = self.on_read_async {
            val = on_read_async().await.map_err(|e| Error::ValueOnRead(e))?;
        }
        if let Some(v) = val {
            self.set_value(v).await?;
        }

        Ok(self.value.clone())
    }

    /// Sets the value of the characteristic.
    pub async fn set_value(&mut self, val: T) -> Result<()> {
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

        let old_val = self.value.clone();
        if let Some(ref mut on_update) = self.on_update {
            on_update(&old_val, &val).map_err(|e| Error::ValueOnUpdate(e))?;
        }
        if let Some(ref mut on_update_async) = self.on_update_async {
            on_update_async(old_val, val.clone())
                .await
                .map_err(|e| Error::ValueOnUpdate(e))?;
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

    /// Returns the [`Unit`](Unit) of the characteristic.
    pub fn get_unit(&self) -> Option<Unit> { self.unit }

    /// Sets the [`Unit`](Unit) of the characteristic.
    pub fn set_unit(&mut self, unit: Option<Unit>) { self.unit = unit; }

    /// Returns the maximum value of the characteristic.
    pub fn get_max_value(&self) -> Option<T> { self.max_value.clone() }

    /// Sets the maximum value of the characteristic.
    pub fn set_max_value(&mut self, val: Option<T>) { self.max_value = val; }

    /// Returns the minimum value of the characteristic.
    pub fn get_min_value(&self) -> Option<T> { self.min_value.clone() }

    /// Sets the minimum value of the characteristic.
    pub fn set_min_value(&mut self, val: Option<T>) { self.min_value = val; }

    /// Returns the step value of the characteristic.
    pub fn get_step_value(&self) -> Option<T> { self.step_value.clone() }

    /// Sets the step value of the characteristic.
    pub fn set_step_value(&mut self, val: Option<T>) { self.step_value = val; }

    /// Returns the maximum length of the characteristic.
    pub fn get_max_len(&self) -> Option<u16> { self.max_len }

    /// Sets the maximum length of the characteristic.
    pub fn set_max_len(&mut self, val: Option<u16>) { self.max_len = val; }

    /// Returns the maximum data length of the characteristic.
    pub fn get_max_data_len(&self) -> Option<u32> { self.max_data_len }

    /// Sets the maximum data length of the characteristic.
    pub fn set_max_data_len(&mut self, val: Option<u32>) { self.max_data_len = val; }

    /// Returns the valid values of the characteristic.
    pub fn get_valid_values(&self) -> Option<Vec<T>> { self.valid_values.clone() }

    /// Sets the valid values of the characteristic.
    pub fn set_valid_values(&mut self, val: Option<Vec<T>>) { self.valid_values = val; }

    /// Returns the valid values range of the characteristic.
    pub fn get_valid_values_range(&self) -> Option<[T; 2]> { self.valid_values_range.clone() }

    /// Sets the valid values range of the characteristic.
    pub fn set_valid_values_range(&mut self, val: Option<[T; 2]>) { self.valid_values_range = val; }

    /// Returns the TTL of the characteristic.
    pub fn get_ttl(&self) -> Option<u64> { self.ttl }

    /// Sets the TTL of the characteristic.
    pub fn set_ttl(&mut self, val: Option<u64>) { self.ttl = val; }

    /// Returns the PID of the characteristic.
    pub fn get_pid(&self) -> Option<u64> { self.pid }

    /// Sets the PID of the characteristic.
    pub fn set_pid(&mut self, val: Option<u64>) { self.pid = val; }

    /// Sets a callback function on a characteristic that is called every time a controller attempts to read its value.
    /// Returning a `Some(T)` from this function changes the value of the characteristic before the controller reads
    /// it so the controller reads the new value.
    pub fn on_read(&mut self, f: Option<impl OnReadFn<T>>) {
        self.on_read = f.map(|f| Box::new(f) as Box<dyn OnReadFn<T>>);
    }

    /// Sets a callback function on a characteristic that is called every time a controller attempts to update its
    /// value. The first argument is a reference to the current value of the characteristic and the second argument is a
    /// reference to the value the controller attempts to change the characteristic's to.
    pub fn on_update(&mut self, f: Option<impl OnUpdateFn<T>>) {
        self.on_update = f.map(|f| Box::new(f) as Box<dyn OnUpdateFn<T>>);
    }

    /// Sets an async callback function on a characteristic that is driven to completion by the async runtime driving
    /// the HAP server every time a controller attempts to read its value. Returning a `Some(T)` from this function
    /// changes the value of the characteristic before the controller reads it so the controller reads the new value.
    pub fn on_read_async(&mut self, f: Option<impl OnReadFuture<T>>) {
        self.on_read_async = f.map(|f| Box::new(f) as Box<dyn OnReadFuture<T>>);
    }

    /// Sets an async callback function on a characteristic that is driven to completion by the async runtime driving
    /// the HAP server every time a controller attempts to update its value. The first argument is a reference to the
    /// current value of the characteristic and the second argument is a reference to the value the controller attempts
    /// to change the characteristic's to.
    pub fn on_update_async(&mut self, f: Option<impl OnUpdateFuture<T>>) {
        self.on_update_async = f.map(|f| Box::new(f) as Box<dyn OnUpdateFuture<T>>);
    }

    /// Sets a pointer to an `EventEmitter` on the characteristic.
    pub(crate) fn set_event_emitter(&mut self, event_emitter: Option<pointer::EventEmitter>) {
        self.event_emitter = event_emitter;
    }
}

impl<T: fmt::Debug + Default + Clone + Serialize + Send + Sync> Serialize for Characteristic<T> {
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
        if let Some(ref ttl) = self.ttl {
            state.serialize_field("TTL", ttl)?;
        }
        if let Some(ref pid) = self.pid {
            state.serialize_field("pid", pid)?;
        }
        state.end()
    }
}

/// Permission of a characteristic.
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
    #[serde(rename = "wr")]
    WriteResponse,
}

/// [`Unit`](Unit) of a characteristic.
#[derive(Debug, Copy, Clone, Serialize)]
pub enum Unit {
    #[serde(rename = "celsius")]
    Celsius,
    #[serde(rename = "fahrenheit")]
    Fahrenheit,
    #[serde(rename = "percentage")]
    Percentage,
    #[serde(rename = "arcdegrees")]
    ArcDegrees,
    #[serde(rename = "lux")]
    Lux,
    #[serde(rename = "seconds")]
    Seconds,
    #[serde(rename = "ppm")]
    PartsPerMillion,
    #[serde(rename = "micrograms/m^3")]
    MicrogramsPerCubicMeter,
}

/// [`Format`](Format) (data type) of a characteristic.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "uint8")]
    UInt8,
    #[serde(rename = "uint16")]
    UInt16,
    #[serde(rename = "uint32")]
    UInt32,
    #[serde(rename = "uint64")]
    UInt64,
    #[serde(rename = "int")]
    Int32,
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "tlv8")]
    Tlv8,
    #[serde(rename = "data")]
    Data,
}

impl Default for Format {
    fn default() -> Format { Format::String }
}

/// [`HapCharacteristic`](HapCharacteristic) is implemented by every HAP characteristic.
#[async_trait]
pub trait HapCharacteristic: HapCharacteristicSetup + erased_serde::Serialize + Send + Sync {
    /// Returns the ID of the characteristic.
    fn get_id(&self) -> u64;
    /// Sets the ID of the characteristic.
    fn set_id(&mut self, id: u64);
    /// Returns the [`HapType`](HapType) of the characteristic.
    fn get_type(&self) -> HapType;
    /// Sets the [`HapType`](HapType) of the characteristic.
    fn set_type(&mut self, hap_type: HapType);
    /// Returns the [`Format`](Format) of the characteristic.
    fn get_format(&self) -> Format;
    /// Sets the [`Format`](Format) of the characteristic.
    fn set_format(&mut self, format: Format);
    /// Returns the [`Perm`](Perm)s of the characteristic.
    fn get_perms(&self) -> Vec<Perm>;
    /// Sets the [`Perm`](Perm)s of the characteristic.
    fn set_perms(&mut self, perms: Vec<Perm>);
    /// Returns the description of the characteristic.
    fn get_description(&self) -> Option<String>;
    /// Sets the description of the characteristic.
    fn set_description(&mut self, description: Option<String>);
    /// Returns the `event_notifications` value of the characteristic.
    fn get_event_notifications(&self) -> Option<bool>;
    /// Sets the `event_notifications` value of the characteristic.
    fn set_event_notifications(&mut self, event_notifications: Option<bool>);
    /// Returns the value of the characteristic.
    async fn get_value(&mut self) -> Result<serde_json::Value>;
    /// Sets the value of the characteristic.
    async fn set_value(&mut self, value: serde_json::Value) -> Result<()>;
    /// Returns the [`Unit`](Unit) of the characteristic.
    fn get_unit(&self) -> Option<Unit>;
    /// Sets the [`Unit`](Unit) of the characteristic.
    fn set_unit(&mut self, unit: Option<Unit>);
    /// Returns the maximum value of the characteristic.
    fn get_max_value(&self) -> Option<serde_json::Value>;
    /// Sets the maximum value of the characteristic.
    fn set_max_value(&mut self, max_value: Option<serde_json::Value>) -> Result<()>;
    /// Returns the minimum value of the characteristic.
    fn get_min_value(&self) -> Option<serde_json::Value>;
    /// Sets the minimum value of the characteristic.
    fn set_min_value(&mut self, min_value: Option<serde_json::Value>) -> Result<()>;
    /// Returns the step value of the characteristic.
    fn get_step_value(&self) -> Option<serde_json::Value>;
    /// Sets the step value of the characteristic.
    fn set_step_value(&mut self, step_value: Option<serde_json::Value>) -> Result<()>;
    /// Returns the maximum length of the characteristic.
    fn get_max_len(&self) -> Option<u16>;
    /// Sets the maximum length of the characteristic.
    fn set_max_len(&mut self, max_len: Option<u16>);
    /// Returns the maximum data length of the characteristic.
    fn get_max_data_len(&self) -> Option<u32>;
    /// Sets the maximum data length of the characteristic.
    fn set_max_data_len(&mut self, max_data_len: Option<u32>);
    /// Returns the valid values of the characteristic.
    fn get_valid_values(&self) -> Option<Vec<serde_json::Value>>;
    /// Sets the valid values of the characteristic.
    fn set_valid_values(&mut self, valid_values: Option<Vec<serde_json::Value>>) -> Result<()>;
    /// Returns the valid values range of the characteristic.
    fn get_valid_values_range(&self) -> Option<[serde_json::Value; 2]>;
    /// Sets the valid values range of the characteristic.
    fn set_valid_values_range(&mut self, valid_values_range: Option<[serde_json::Value; 2]>) -> Result<()>;
    /// Returns the TTL of the characteristic.
    fn get_ttl(&self) -> Option<u64>;
    /// Sets the TTL of the characteristic.
    fn set_ttl(&mut self, ttl: Option<u64>);
    /// Returns the PID of the characteristic.
    fn get_pid(&self) -> Option<u64>;
    /// Sets the PID of the characteristic.
    fn set_pid(&mut self, pid: Option<u64>);
}

serialize_trait_object!(HapCharacteristic);

/// [`HapCharacteristicSetup`](HapCharacteristicSetup) is implemented by every HAP characteristic to provide helper
/// methods used by the HAP server for setup purposes. It's not meant to be used by a consumer of the library.
pub trait HapCharacteristicSetup {
    /// Sets a pointer to an `EventEmitter` on the characteristic.
    fn set_event_emitter(&mut self, event_emitter: Option<pointer::EventEmitter>);
}

/// [`OnReadFn`](OnReadFn) represents a callback function to be set on a characteristic that is called every time a
/// controller attempts to read its value. Returning a `Some(T)` from this function changes the value of the
/// characteristic before the controller reads it so the controller reads the new value.
pub trait OnReadFn<T: Default + Clone + Serialize + Send + Sync>:
    Fn() -> std::result::Result<Option<T>, Box<dyn std::error::Error + Send + Sync>> + 'static + Send + Sync
{
}
impl<F, T: Default + Clone + Serialize + Send + Sync> OnReadFn<T> for F where
    F: Fn() -> std::result::Result<Option<T>, Box<dyn std::error::Error + Send + Sync>> + 'static + Send + Sync
{
}

/// [`OnUpdateFn`](OnUpdateFn) represents a callback function to be set on a characteristic that is called every time a
/// controller attempts to update its value. The first argument is a reference to the current value of the
/// characteristic and the second argument is a reference to the value the controller attempts to change the
/// characteristic's to.
pub trait OnUpdateFn<T: Default + Clone + Serialize + Send + Sync>:
    FnMut(&T, &T) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> + 'static + Send + Sync
{
}
impl<F, T: Default + Clone + Serialize + Send + Sync> OnUpdateFn<T> for F where
    F: FnMut(&T, &T) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> + 'static + Send + Sync
{
}

/// [`OnReadFuture`](OnReadFuture) represents an async callback function to be set on a characteristic that is driven to
/// completion by the async runtime driving the HAP server every time a controller attempts to read its value. Returning
/// a `Some(T)` from this function changes the value of the characteristic before the controller reads it so the
/// controller reads the new value.
pub trait OnReadFuture<T: Default + Clone + Serialize + Send + Sync>:
    FnMut() -> BoxFuture<'static, std::result::Result<Option<T>, Box<dyn std::error::Error + Send + Sync>>>
    + 'static
    + Send
    + Sync
{
}
impl<F, T: Default + Clone + Serialize + Send + Sync> OnReadFuture<T> for F where
    F: FnMut() -> BoxFuture<'static, std::result::Result<Option<T>, Box<dyn std::error::Error + Send + Sync>>>
        + 'static
        + Send
        + Sync
{
}

/// [`OnUpdateFuture`](OnUpdateFuture) represents an async callback function to be set on a characteristic that is
/// driven to completion by the async runtime driving the HAP server every time a controller attempts to update its
/// value. The first argument is a reference to the current value of the characteristic and the second argument is a
/// reference to the value the controller attempts to change the characteristic's to.
pub trait OnUpdateFuture<T: Default + Clone + Serialize + Send + Sync>:
    FnMut(T, T) -> BoxFuture<'static, std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>>
    + 'static
    + Send
    + Sync
{
}
impl<F, T: Default + Clone + Serialize + Send + Sync> OnUpdateFuture<T> for F where
    F: FnMut(T, T) -> BoxFuture<'static, std::result::Result<(), Box<dyn std::error::Error + Send + Sync>>>
        + 'static
        + Send
        + Sync
{
}

/// Trait containing the [`OnReadFn`](OnReadFn) and [`OnUpdateFn`](OnUpdateFn) callback functions.
pub trait CharacteristicCallbacks<T: fmt::Debug + Default + Clone + Serialize + Send + Sync> {
    /// Sets a callback function on a characteristic that is called every time a controller attempts to read its value.
    /// Returning a `Some(T)` from this function changes the value of the characteristic before the controller reads
    /// it so the controller reads the new value.
    fn on_read(&mut self, f: Option<impl OnReadFn<T>>);
    /// Sets a callback function on a characteristic that is called every time a controller attempts to update its
    /// value. The first argument is a reference to the current value of the characteristic and the second argument is a
    /// reference to the value the controller attempts to change the characteristic's to.
    fn on_update(&mut self, f: Option<impl OnUpdateFn<T>>);
}

/// Trait containing the [`OnReadFuture`](OnReadFuture) and [`OnUpdateFuture`](OnUpdateFuture) callback functions.
pub trait AsyncCharacteristicCallbacks<T: fmt::Debug + Default + Clone + Serialize + Send + Sync> {
    /// Sets an async callback function on a characteristic that is driven to completion by the async runtime driving
    /// the HAP server every time a controller attempts to read its value. Returning a `Some(T)` from this function
    /// changes the value of the characteristic before the controller reads it so the controller reads the new value.
    fn on_read_async(&mut self, f: Option<impl OnReadFuture<T>>);
    /// Sets an async callback function on a characteristic that is driven to completion by the async runtime driving
    /// the HAP server every time a controller attempts to update its value. The first argument is a reference to the
    /// current value of the characteristic and the second argument is a reference to the value the controller attempts
    /// to change the characteristic's to.
    fn on_update_async(&mut self, f: Option<impl OnUpdateFuture<T>>);
}

#[async_trait]
impl<T: fmt::Debug + Default + Clone + Serialize + Send + Sync> HapCharacteristic for Characteristic<T>
where
    for<'de> T: Deserialize<'de>,
{
    fn get_id(&self) -> u64 { Characteristic::get_id(self) }

    fn set_id(&mut self, id: u64) { Characteristic::set_id(self, id) }

    fn get_type(&self) -> HapType { Characteristic::get_type(self) }

    fn set_type(&mut self, hap_type: HapType) { Characteristic::set_type(self, hap_type) }

    fn get_format(&self) -> Format { Characteristic::get_format(self) }

    fn set_format(&mut self, format: Format) { Characteristic::set_format(self, format) }

    fn get_perms(&self) -> Vec<Perm> { Characteristic::get_perms(self) }

    fn set_perms(&mut self, perms: Vec<Perm>) { Characteristic::set_perms(self, perms) }

    fn get_description(&self) -> Option<String> { Characteristic::get_description(self) }

    fn set_description(&mut self, description: Option<String>) { Characteristic::set_description(self, description) }

    fn get_event_notifications(&self) -> Option<bool> { Characteristic::get_event_notifications(self) }

    fn set_event_notifications(&mut self, event_notifications: Option<bool>) {
        Characteristic::set_event_notifications(self, event_notifications)
    }

    async fn get_value(&mut self) -> Result<serde_json::Value> {
        let value = Characteristic::get_value(self).await?;
        Ok(json!(value))
    }

    async fn set_value(&mut self, value: serde_json::Value) -> Result<()> {
        let v;
        // for whatever reason, the controller is setting boolean values either as a boolean or as an integer
        if self.format == Format::Bool && value.is_number() {
            let num_v: u8 = serde_json::from_value(value)?;
            if num_v == 0 {
                v = serde_json::from_value(json!(false))?;
            } else if num_v == 1 {
                v = serde_json::from_value(json!(true))?;
            } else {
                return Err(Error::InvalidValue(Characteristic::get_format(self)));
            }
        } else {
            v = serde_json::from_value(value).map_err(|_| Error::InvalidValue(Characteristic::get_format(self)))?;
        }
        Characteristic::set_value(self, v).await
    }

    fn get_unit(&self) -> Option<Unit> { Characteristic::get_unit(self) }

    fn set_unit(&mut self, unit: Option<Unit>) { Characteristic::set_unit(self, unit) }

    fn get_max_value(&self) -> Option<serde_json::Value> { Characteristic::get_max_value(self).map(|v| json!(v)) }

    fn set_max_value(&mut self, max_value: Option<serde_json::Value>) -> Result<()> {
        Characteristic::set_max_value(self, match max_value {
            Some(v) =>
                Some(serde_json::from_value(v).map_err(|_| Error::InvalidValue(Characteristic::get_format(self)))?),
            None => None,
        });

        Ok(())
    }

    fn get_min_value(&self) -> Option<serde_json::Value> { Characteristic::get_min_value(self).map(|v| json!(v)) }

    fn set_min_value(&mut self, min_value: Option<serde_json::Value>) -> Result<()> {
        Characteristic::set_min_value(self, match min_value {
            Some(v) =>
                Some(serde_json::from_value(v).map_err(|_| Error::InvalidValue(Characteristic::get_format(self)))?),
            None => None,
        });

        Ok(())
    }

    fn get_step_value(&self) -> Option<serde_json::Value> { Characteristic::get_step_value(self).map(|v| json!(v)) }

    fn set_step_value(&mut self, step_value: Option<serde_json::Value>) -> Result<()> {
        Characteristic::set_step_value(self, match step_value {
            Some(v) =>
                Some(serde_json::from_value(v).map_err(|_| Error::InvalidValue(Characteristic::get_format(self)))?),
            None => None,
        });

        Ok(())
    }

    fn get_max_len(&self) -> Option<u16> { Characteristic::get_max_len(self) }

    fn set_max_len(&mut self, max_len: Option<u16>) { Characteristic::set_max_len(self, max_len) }

    fn get_max_data_len(&self) -> Option<u32> { Characteristic::get_max_data_len(self) }

    fn set_max_data_len(&mut self, max_data_len: Option<u32>) { Characteristic::set_max_data_len(self, max_data_len) }

    fn get_valid_values(&self) -> Option<Vec<serde_json::Value>> {
        Characteristic::get_valid_values(self).map(|v| v.into_iter().map(|v| json!(v)).collect())
    }

    fn set_valid_values(&mut self, valid_values: Option<Vec<serde_json::Value>>) -> Result<()> {
        Characteristic::set_valid_values(self, match valid_values {
            Some(v) => Some(
                v.into_iter()
                    .map(|v| {
                        serde_json::from_value(v).map_err(|_| Error::InvalidValue(Characteristic::get_format(self)))
                    })
                    .collect::<Result<Vec<T>>>()?,
            ),
            None => None,
        });

        Ok(())
    }

    fn get_valid_values_range(&self) -> Option<[serde_json::Value; 2]> {
        Characteristic::get_valid_values_range(self).map(|v| [json!(v[0]), json!(v[1])])
    }

    fn set_valid_values_range(&mut self, valid_values_range: Option<[serde_json::Value; 2]>) -> Result<()> {
        Characteristic::set_valid_values_range(self, match valid_values_range {
            Some([start, end]) => Some(Result::<[T; 2]>::Ok([
                serde_json::from_value(start).map_err(|_| Error::InvalidValue(Characteristic::get_format(self)))?,
                serde_json::from_value(end).map_err(|_| Error::InvalidValue(Characteristic::get_format(self)))?,
            ])?),
            None => None,
        });

        Ok(())
    }

    fn get_ttl(&self) -> Option<u64> { Characteristic::get_ttl(self) }

    fn set_ttl(&mut self, ttl: Option<u64>) { Characteristic::set_ttl(self, ttl) }

    fn get_pid(&self) -> Option<u64> { Characteristic::get_pid(self) }

    fn set_pid(&mut self, pid: Option<u64>) { Characteristic::set_pid(self, pid) }
}

impl<T: fmt::Debug + Default + Clone + Serialize + Send + Sync> HapCharacteristicSetup for Characteristic<T>
where
    for<'de> T: Deserialize<'de>,
{
    fn set_event_emitter(&mut self, event_emitter: Option<pointer::EventEmitter>) {
        Characteristic::set_event_emitter(self, event_emitter)
    }
}

impl<T: fmt::Debug + Default + Clone + Serialize + Send + Sync> CharacteristicCallbacks<T> for Characteristic<T>
where
    for<'de> T: Deserialize<'de>,
{
    fn on_read(&mut self, f: Option<impl OnReadFn<T>>) { Characteristic::on_read(self, f) }

    fn on_update(&mut self, f: Option<impl OnUpdateFn<T>>) { Characteristic::on_update(self, f) }
}

impl<T: fmt::Debug + Default + Clone + Serialize + Send + Sync> AsyncCharacteristicCallbacks<T> for Characteristic<T>
where
    for<'de> T: Deserialize<'de>,
{
    fn on_read_async(&mut self, f: Option<impl OnReadFuture<T>>) { Characteristic::on_read_async(self, f) }

    fn on_update_async(&mut self, f: Option<impl OnUpdateFuture<T>>) { Characteristic::on_update_async(self, f) }
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

            ttl: None,
            pid: None,

            on_read: None,
            on_update: None,
            on_read_async: None,
            on_update_async: None,

            event_emitter: None,
        };
        let json = serde_json::to_string(&characteristic).unwrap();
        assert_eq!(json, "{\"iid\":1,\"type\":\"C1\",\"format\":\"uint16\",\"perms\":[\"pr\",\"ev\"],\"description\":\"Acme Tilt Angle\",\"ev\":true,\"value\":123,\"unit\":\"arcdegrees\",\"maxValue\":360,\"minValue\":0,\"minStep\":1,\"valid-values-range\":[0,360]}".to_string());
    }
}
