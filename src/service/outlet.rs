use characteristic;
use service::ServiceT;
use hap_type;

#[derive(Default)]
pub struct Outlet {
    id: u64,
    hap_type: hap_type::HAPType,

    on: characteristic::on::On,
    outlet_in_use: characteristic::outlet_in_use::OutletInUse,
}

impl ServiceT for Outlet {
    fn get_characteristics(&self) -> Vec<&characteristic::CharacteristicT> {
        vec![&self.on, &self.outlet_in_use]
    }
}

pub fn new() -> Outlet {
    Outlet {
        hap_type: "47".into(),
        on: characteristic::on::new(),
        outlet_in_use: characteristic::outlet_in_use::new(),
        ..Default::default()
    }
}
