use std::error::Error;
use hap_type;

pub mod on;
pub mod outlet_in_use;

#[derive(Default)]
pub struct Characteristic<T: Default> {
    id: u64,
    hap_type: hap_type::HAPType,
    perms: Vec<Perm>,
    description: String,

    value: T,
    unit: Option<Unit>,

    max_len: Option<u32>,
    max_value: Option<T>,
    min_value: Option<T>,
    step_value: Option<T>,
}

impl<T: Default> Characteristic<T> {
    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /*fn setValue(&mut self, val: T) -> Result<(), Error> {
        if let Some(max) = self.max_value {
            if val > max {
                return Err("value above max_value".into());
            }
        }
        if let Some(min) = self.min_value {
            if val < min {
                return Err("value below min_value".into());
            }
        }

        self.value = val;
    }*/

    fn set_min_value(&mut self, val: T) {
        self.min_value = Some(val);
    }

    fn set_max_value(&mut self, val: T) {
        self.max_value = Some(val);
    }

    fn set_step_value(&mut self, val: T) {
        self.step_value = Some(val);
    }
}

/*trait UpdateValueFromCon {
    fn updateValueFromCon();
}

impl<T> UpdateValueFromCon for Characteristic<T> {
    fn updateValueFromCon(Json) -> Result<(), Error> {

    }
}*/

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
