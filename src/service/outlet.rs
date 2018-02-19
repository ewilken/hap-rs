use serde_json;

use service::HapService;
use characteristic::{HapCharacteristic, on, outlet_in_use};
use hap_type::HapType;

#[derive(Default)]
pub struct Outlet {
    id: u64,
    hap_type: HapType,

    pub on: on::On,
    pub outlet_in_use: outlet_in_use::OutletInUse,
}

impl HapService for Outlet {
    fn get_id(&self) -> &u64 {
        &self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_type(&self) -> &HapType {
        &self.hap_type
    }

    fn get_characteristics(&self) -> Vec<&HapCharacteristic> {
        vec![
            &self.on,
            &self.outlet_in_use,
        ]
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut HapCharacteristic> {
        vec![
            &mut self.on,
            &mut self.outlet_in_use,
        ]
    }

    fn to_json(&self) -> serde_json::Value {
        let characteristics: Vec<serde_json::Value> = self.get_characteristics().iter().map(|c| c.to_json()).collect();
        json!({
            "type": self.get_type(),
            "iid": self.get_id(),
            "characteristics": characteristics,
        })
    }
}

pub fn new() -> Outlet {
    Outlet {
        hap_type: "47".into(),
        on: on::new(),
        outlet_in_use: outlet_in_use::new(),
        ..Default::default()
    }
}
