use serde_json;

use characteristic::HapCharacteristic;
use hap_type::HapType;

pub mod accessory_information;
pub mod outlet;

pub trait HapService {
    fn get_id(&self) -> &u64;
    fn set_id(&mut self, id: u64);
    fn get_type(&self) -> &HapType;
    fn get_characteristics(&self) -> Vec<&HapCharacteristic>;
    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic>;
    fn to_json(&self) -> serde_json::Value;
}
