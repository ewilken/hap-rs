use std::{rc::Rc, cell::RefCell};

use serde::{ser::{Serialize, Serializer, SerializeStruct}, Deserialize};
use serde_json;
use erased_serde;

use HapType;
use event::{Event, EmitterPtr};

use Error;

mod includes;
pub use characteristic::includes::*;

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

    readable: Option<Box<Readable<T>>>,
    updatable: Option<Box<Updatable<T>>>,

    event_emitter: Option<EmitterPtr>,
}

#[derive(Clone, Default)]
pub struct Characteristic<T: Default + Clone + Serialize> {
    pub inner: Rc<RefCell<Inner<T>>>,
}

impl<T: Default + Clone + Serialize> Characteristic<T> where for<'de> T: Deserialize<'de> {
    pub fn new(inner: Inner<T>) -> Characteristic<T> {
        Characteristic { inner: Rc::new(RefCell::new(inner)) }
    }

    pub fn get_id(&self) -> Result<u64, Error> {
        Ok(self.inner.try_borrow()?.id)
    }

    pub fn set_id(&mut self, id: u64) -> Result<(), Error> {
        self.inner.try_borrow_mut()?.id = id;
        Ok(())
    }

    pub fn set_accessory_id(&mut self, accessory_id: u64) -> Result<(), Error> {
        self.inner.try_borrow_mut()?.accessory_id = accessory_id;
        Ok(())
    }

    pub fn get_type(&self) -> Result<HapType, Error> {
        Ok(self.inner.try_borrow()?.hap_type)
    }

    pub fn get_format(&self) -> Result<Format, Error> {
        Ok(self.inner.try_borrow()?.format)
    }

    pub fn get_perms(&self) -> Result<Vec<Perm>, Error> {
        Ok(self.inner.try_borrow()?.perms.clone())
    }

    pub fn set_description(&mut self, description: Option<String>) -> Result<(), Error> {
        self.inner.try_borrow_mut()?.description = description;
        Ok(())
    }

    pub fn get_event_notifications(&self) -> Result<Option<bool>, Error> {
        Ok(self.inner.try_borrow()?.event_notifications)
    }

    pub fn set_event_notifications(&mut self, event_notifications: Option<bool>) -> Result<(), Error> {
        self.inner.try_borrow_mut()?.event_notifications = event_notifications;
        Ok(())
    }

    pub fn get_value(&mut self) -> Result<T, Error> {
        let mut val = None;
        {
            let mut inner = self.inner.try_borrow_mut()?;
            let hap_type = inner.hap_type;
            if let Some(ref mut readable) = inner.readable {
                val = readable.on_read(hap_type);
            }
        }
        if let Some(v) = val {
            self.set_value(v)?;
        }

        Ok(self.inner.try_borrow()?.value.clone())
    }

    pub fn set_value(&mut self, val: T) -> Result<(), Error> {
        // TODO - check for min/max on types implementing PartialOrd
        // if let Some(ref max) = self.inner.try_borrow()?.max_value {
        //     if &val > max {
        //         return Err(Error::new_io("value above max_value"));
        //     }
        // }
        // if let Some(ref min) = self.inner.try_borrow()?.min_value {
        //     if &val < min {
        //         return Err(Error::new_io("value below min_value"));
        //     }
        // }

        {
            let mut inner = self.inner.try_borrow_mut()?;
            let old_val = inner.value.clone();
            let hap_type = inner.hap_type;
            if let Some(ref mut updatable) = inner.updatable {
                updatable.on_update(&old_val, &val, hap_type);
            }
        }

        {
            let inner = self.inner.try_borrow()?;
            if inner.event_notifications == Some(true) {
                if let Some(ref event_emitter) = inner.event_emitter {
                    event_emitter.try_borrow()?.emit(Event::CharacteristicValueChanged {
                        aid: inner.accessory_id,
                        iid: inner.id,
                        value: json!(&val),
                    });
                }
            }
        }

        self.inner.try_borrow_mut()?.value = val;

        Ok(())
    }

    pub fn get_unit(&self) -> Result<Option<Unit>, Error> {
        Ok(self.inner.try_borrow()?.unit)
    }

    pub fn get_max_value(&self) -> Result<Option<T>, Error> {
        Ok(self.inner.try_borrow()?.max_value.clone())
    }

    pub fn set_max_value(&mut self, val: Option<T>) -> Result<(), Error> {
        self.inner.try_borrow_mut()?.max_value = val;
        Ok(())
    }

    pub fn get_min_value(&self) -> Result<Option<T>, Error> {
        Ok(self.inner.try_borrow()?.min_value.clone())
    }

    pub fn set_min_value(&mut self, val: Option<T>) -> Result<(), Error> {
        self.inner.try_borrow_mut()?.min_value = val;
        Ok(())
    }

    pub fn get_step_value(&self) -> Result<Option<T>, Error> {
        Ok(self.inner.try_borrow()?.step_value.clone())
    }

    pub fn set_step_value(&mut self, val: Option<T>) -> Result<(), Error> {
        self.inner.try_borrow_mut()?.step_value = val;
        Ok(())
    }

    pub fn get_max_len(&self) -> Result<Option<u16>, Error> {
        Ok(self.inner.try_borrow()?.max_len)
    }

    pub fn set_readable(&mut self, readable: impl Readable<T> + 'static) -> Result<(), Error> {
        self.inner.try_borrow_mut()?.readable = Some(Box::new(readable));
        Ok(())
    }

    pub fn set_updatable(&mut self, updatable: impl Updatable<T> + 'static) -> Result<(), Error> {
        self.inner.try_borrow_mut()?.updatable = Some(Box::new(updatable));
        Ok(())
    }

    pub fn set_event_emitter(&mut self, event_emitter: Option<EmitterPtr>) -> Result<(), Error> {
        self.inner.try_borrow_mut()?.event_emitter = event_emitter;
        Ok(())
    }
}

impl<T: Default + Clone + Serialize> Serialize for Characteristic<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("Characteristic", 15)?;
        let inner = self.inner.try_borrow()
            .expect("couldn't access Characteristic inner");
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

pub trait HapCharacteristic: erased_serde::Serialize {
    fn get_id(&self) -> Result<u64, Error>;
    fn set_id(&mut self, id: u64) -> Result<(), Error>;
    fn set_accessory_id(&mut self, accessory_id: u64) -> Result<(), Error>;
    fn get_type(&self) -> Result<HapType, Error>;
    fn get_format(&self) -> Result<Format, Error>;
    fn get_perms(&self) -> Result<Vec<Perm>, Error>;
    fn get_event_notifications(&self) -> Result<Option<bool>, Error>;
    fn set_event_notifications(&mut self, event_notifications: Option<bool>) -> Result<(), Error>;
    fn get_value(&mut self) -> Result<serde_json::Value, Error>;
    fn set_value(&mut self, value: serde_json::Value) -> Result<(), Error>;
    fn get_unit(&self) -> Result<Option<Unit>, Error>;
    fn get_max_value(&self) -> Result<Option<serde_json::Value>, Error>;
    fn get_min_value(&self) -> Result<Option<serde_json::Value>, Error>;
    fn get_step_value(&self) -> Result<Option<serde_json::Value>, Error>;
    fn get_max_len(&self) -> Result<Option<u16>, Error>;
    fn set_event_emitter(&mut self, event_emitter: Option<EmitterPtr>) -> Result<(), Error>;
}

serialize_trait_object!(HapCharacteristic);

impl<T: Default + Clone + Serialize> HapCharacteristic for Characteristic<T> where for<'de> T: Deserialize<'de> {
    fn get_id(&self) -> Result<u64, Error> {
        self.get_id()
    }

    fn set_id(&mut self, id: u64) -> Result<(), Error> {
        self.set_id(id)
    }

    fn set_accessory_id(&mut self, accessory_id: u64) -> Result<(), Error> {
        self.set_accessory_id(accessory_id)
    }

    fn get_type(&self) -> Result<HapType, Error> {
        self.get_type()
    }

    fn get_format(&self) -> Result<Format, Error> {
        self.get_format()
    }

    fn get_perms(&self) -> Result<Vec<Perm>, Error> {
        self.get_perms()
    }

    fn get_event_notifications(&self) -> Result<Option<bool>, Error> {
        self.get_event_notifications()
    }

    fn set_event_notifications(&mut self, event_notifications: Option<bool>) -> Result<(), Error> {
        self.set_event_notifications(event_notifications)
    }

    fn get_value(&mut self) -> Result<serde_json::Value, Error> {
        Ok(json!(self.get_value()?))
    }

    fn set_value(&mut self, value: serde_json::Value) -> Result<(), Error> {
        let v;
        // the controller is setting boolean values
        // either as a boolean or as an integer
        if self.inner.try_borrow()?.format == Format::Bool && value.is_number() {
            let num_v: u8 = serde_json::from_value(value)?;
            if num_v == 0 {
                v = serde_json::from_value(json!(false))?;
            } else if num_v == 1 {
                v = serde_json::from_value(json!(true))?;
            } else {
                return Err(Error::new_io("invalid value for bool characteristic"));
            }
        } else {
            v = serde_json::from_value(value)?;
        }
        self.set_value(v)
    }

    fn get_unit(&self) -> Result<Option<Unit>, Error> {
        self.get_unit()
    }

    fn get_max_value(&self) -> Result<Option<serde_json::Value>, Error> {
        Ok(match self.get_max_value()? {
            Some(v) => Some(json!(v)),
            None => None,
        })
    }

    fn get_min_value(&self) -> Result<Option<serde_json::Value>, Error> {
        Ok(match self.get_min_value()? {
            Some(v) => Some(json!(v)),
            None => None,
        })
    }

    fn get_step_value(&self) -> Result<Option<serde_json::Value>, Error> {
        Ok(match self.get_step_value()? {
            Some(v) => Some(json!(v)),
            None => None,
        })
    }

    fn get_max_len(&self) -> Result<Option<u16>, Error> {
        self.get_max_len()
    }

    fn set_event_emitter(&mut self, event_emitter: Option<EmitterPtr>) -> Result<(), Error> {
        self.set_event_emitter(event_emitter)
    }
}

pub trait Readable<T: Default + Serialize> {
    fn on_read(&mut self, hap_type: HapType) -> Option<T>;
}

pub trait Updatable<T: Default + Serialize> {
    fn on_update(&mut self, old_val: &T, new_val: &T, hap_type: HapType);
}

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
    fn default() -> Format {
        Format::String
    }
}
