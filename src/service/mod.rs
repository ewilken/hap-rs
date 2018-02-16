use serde_json;

use characteristic::CharacteristicT;
use hap_type::HapType;

pub mod accessory_information;
pub mod outlet;

#[derive(Default)]
pub struct Service {
    pub id: u64,
    pub hap_type: HapType,
    pub characteristics: Vec<Box<CharacteristicT>>,
}

impl Service {
    pub fn as_json(&self) -> serde_json::Value {
        let characteristics: Vec<serde_json::Value> = self.characteristics.iter().map(|c| c.as_json()).collect();
        json!({
            "type": self.hap_type,
            "iid": self.id,
            "characteristics": characteristics,
        })
    }
}
