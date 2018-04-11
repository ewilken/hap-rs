use serde::ser::{Serialize, Serializer, SerializeStruct};

use characteristic::HapCharacteristic;
use hap_type::HapType;

pub mod accessory_information;
pub mod outlet;

pub trait HapService {
    fn get_id(&self) -> u64;
    fn set_id(&mut self, id: u64);
    fn get_type(&self) -> &HapType;
    fn get_hidden(&self) -> bool;
    fn get_primary(&self) -> bool;
    fn get_characteristics(&self) -> Vec<&HapCharacteristic>;
    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic>;
}

pub struct Service<T: HapService> {
    pub inner: T,
}

impl<T: HapService> Service<T> {
    fn new(inner: T) -> Service<T> {
        Service { inner }
    }
}

impl<T: HapService> Serialize for Service<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // TODO - look into that "linked" array
        state.end()
    }
}

impl<T: HapService> HapService for Service<T> {
    fn get_id(&self) -> u64 {
        self.inner.get_id()
    }

    fn set_id(&mut self, id: u64) {
        self.inner.set_id(id)
    }

    fn get_type(&self) -> &HapType {
        self.inner.get_type()
    }

    fn get_hidden(&self) -> bool {
        self.inner.get_hidden()
    }

    fn get_primary(&self) -> bool {
        self.inner.get_primary()
    }

    fn get_characteristics(&self) -> Vec<&HapCharacteristic> {
        self.inner.get_characteristics()
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> {
        self.inner.get_mut_characteristics()
    }
}
