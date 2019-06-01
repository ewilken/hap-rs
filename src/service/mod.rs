use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{characteristic::HapCharacteristic, HapType};

mod includes;

pub use crate::service::includes::*;

/// `HapService` is implemented by the inner type of every `Service`.
pub trait HapService {
    /// Returns the ID of a Service.
    fn get_id(&self) -> u64;
    /// Sets the ID of a Service.
    fn set_id(&mut self, id: u64);
    /// Returns the `HapType` of a Service.
    fn get_type(&self) -> HapType;
    /// Returns the hidden value of a Service.
    fn get_hidden(&self) -> bool;
    /// Sets the hidden value of a Service.
    fn set_hidden(&mut self, hidden: bool);
    /// Returns the primary value of a Service.
    fn get_primary(&self) -> bool;
    /// Sets the primary value of a Service.
    fn set_primary(&mut self, primary: bool);
    /// Returns references to the Characteristics of a Service.
    fn get_characteristics(&self) -> Vec<&HapCharacteristic>;
    /// Returns mutable references to the Characteristics of a Service.
    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic>;
}

/// A Service. Services group functionality in order to provide context. They are comprised of
/// characteristics.
pub struct Service<T: HapService> {
    pub inner: T,
}

impl<T: HapService> Service<T> {
    /// Creates a new `Service`.
    fn new(inner: T) -> Service<T> { Service { inner } }
}

impl<T: HapService> Serialize for Service<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}

impl<T: HapService> HapService for Service<T> {
    fn get_id(&self) -> u64 { self.inner.get_id() }

    fn set_id(&mut self, id: u64) { self.inner.set_id(id) }

    fn get_type(&self) -> HapType { self.inner.get_type() }

    fn get_hidden(&self) -> bool { self.inner.get_hidden() }

    fn set_hidden(&mut self, hidden: bool) { self.inner.set_hidden(hidden) }

    fn get_primary(&self) -> bool { self.inner.get_primary() }

    fn set_primary(&mut self, primary: bool) { self.inner.set_primary(primary) }

    fn get_characteristics(&self) -> Vec<&HapCharacteristic> { self.inner.get_characteristics() }

    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> { self.inner.get_mut_characteristics() }
}
