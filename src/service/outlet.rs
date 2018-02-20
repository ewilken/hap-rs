use service::{HapService, Service};
use characteristic::{HapCharacteristic, on, outlet_in_use};
use hap_type::HapType;

pub type Outlet = Service<OutletInner>;

impl Default for Outlet {
    fn default() -> Outlet { new() }
}

#[derive(Default)]
pub struct OutletInner {
    id: u64,
    hap_type: HapType,
    hidden: bool,
    primary: bool,

    pub on: on::On,
    pub outlet_in_use: outlet_in_use::OutletInUse,
}

impl HapService for OutletInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_type(&self) -> &HapType {
        &self.hap_type
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn get_primary(&self) -> bool {
        self.primary
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
}

pub fn new() -> Outlet {
    Outlet::new(OutletInner {
        hap_type: "47".into(),
        on: on::new(),
        outlet_in_use: outlet_in_use::new(),
        ..Default::default()
    })
}
