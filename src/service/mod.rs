use erased_serde::serialize_trait_object;

use crate::{characteristic::HapCharacteristic, HapType};

mod generated;

pub use crate::service::generated::*;

/// `HapService` is implemented by the inner type of every `Service`.
pub trait HapService: erased_serde::Serialize + Send + Sync {
    /// Returns the ID of a Service.
    fn get_id(&self) -> u64;
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
    /// Returns a reference to a specific Characteristic of the Service if it's present on it.
    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic>;
    /// Returns a mutable reference to a specific Characteristic of the Service if it's present on it.
    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic>;
    /// Returns references to the Characteristics of a Service.
    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic>;
    /// Returns mutable references to the Characteristics of a Service.
    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic>;
}

serialize_trait_object!(HapService);
