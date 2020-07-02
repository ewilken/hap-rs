use crate::{characteristic::HapCharacteristic, HapType};

mod generated;

pub use crate::service::generated::*;

/// `HapService` is implemented by the inner type of every `Service`.
pub trait HapService {
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
    /// Returns references to the Characteristics of a Service.
    fn get_characteristics(&self) -> Vec<&(dyn HapCharacteristic + Send + Sync)>;
    /// Returns mutable references to the Characteristics of a Service.
    fn get_mut_characteristics(&mut self) -> Vec<&mut (dyn HapCharacteristic + Send + Sync)>;
}
