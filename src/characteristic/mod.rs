use std::io::{Error/*, ErrorKind*/};
use hap_type::HAPType;

pub mod firmware_revision;
pub mod identify;
pub mod manufacturer;
pub mod model;
pub mod name;
pub mod on;
pub mod outlet_in_use;
pub mod serial_number;

#[derive(Default)]
pub struct Characteristic<T: Default> {
    id: u64,
    hap_type: HAPType,
    perms: Vec<Perm>,
    description: Option<String>,

    value: T,
    unit: Option<Unit>,

    max_len: Option<u32>,
    max_value: Option<T>,
    min_value: Option<T>,
    step_value: Option<T>,
}

impl<T: Default> Characteristic<T> {
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

        self.value = val;

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

pub trait CharacteristicT {

}

impl<T: Default> CharacteristicT for Characteristic<T> {

}

enum Perm {
    PairedRead,
    PairedWrite,
    Events,
    AdditionalAuthorization,
    TimedWrite,
    Hidden,
}

impl Perm {
    fn as_str(&self) -> &str {
        match self {
            &Perm::PairedRead => "pr",
            &Perm::PairedWrite => "pw",
            &Perm::Events => "ev",
            &Perm::AdditionalAuthorization => "aa",
            &Perm::TimedWrite => "tw",
            &Perm::Hidden => "hd",
        }
    }
}

enum Unit {
    Percentage,
    ArcDegrees,
    Celsius,
    Lux,
    Seconds,
}

impl Unit {
    fn as_str(&self) -> &str {
        match self {
            &Unit::Percentage => "percentage",
            &Unit::ArcDegrees => "arcdegrees",
            &Unit::Celsius => "celsius",
            &Unit::Lux => "lux",
            &Unit::Seconds => "seconds",
        }
    }
}
