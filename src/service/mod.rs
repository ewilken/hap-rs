use erased_serde::serialize_trait_object;

use crate::{characteristic::HapCharacteristic, HapType};

mod generated;

pub use crate::service::generated::*;

/// [`HapService`](HapService) is implemented by every HAP service.
pub trait HapService: erased_serde::Serialize + Send + Sync {
    /// Returns the ID of the service.
    fn get_id(&self) -> u64;
    /// Sets the ID of the service.
    fn set_id(&mut self, id: u64);
    /// Returns the [`HapType`](HapType) of the service.
    fn get_type(&self) -> HapType;
    /// Sets the [`HapType`](HapType) of the service.
    fn set_type(&mut self, hap_type: HapType);
    /// Returns the `hidden` value of the service.
    fn get_hidden(&self) -> bool;
    /// Sets the `hidden` value of the service.
    fn set_hidden(&mut self, hidden: bool);
    /// Returns the `primary` value of the service.
    fn get_primary(&self) -> bool;
    /// Sets the `primary` value of the service.
    fn set_primary(&mut self, primary: bool);
    /// Returns the `linked_services` value of the service.
    fn get_linked_services(&self) -> Vec<u64>;
    /// Sets the `linked_services` value of the service.
    fn set_linked_services(&mut self, linked_services: Vec<u64>);
    /// Returns a reference to a specific characteristic of the service if it's present on it.
    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic>;
    /// Returns a mutable reference to a specific characteristic of the service if it's present on it.
    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic>;
    /// Returns references to all characteristics of the service.
    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic>;
    /// Returns mutable references to all characteristics of the service.
    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic>;
}

serialize_trait_object!(HapService);
