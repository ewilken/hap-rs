use characteristic;
use hap_type;

struct Outlet {
    id: u64,
    hap_type: hap_type::HAPType,

    on: characteristic::on::On,
    outlet_in_use: characteristic::outlet_in_use::OutletInUse,
}

fn new(id: u64) -> Outlet {
    Outlet {
        id,
        hap_type: "47".into(),
        on: characteristic::on::new(),
        outlet_in_use: characteristic::outlet_in_use::new(),
    }
}
