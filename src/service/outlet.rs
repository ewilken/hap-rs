use service::Service;
use characteristic::{on, outlet_in_use};

pub type Outlet = Service;

pub fn new() -> Outlet {
    Outlet {
        hap_type: "47".into(),
        characteristics: vec![
            Box::new(on::new()),
            Box::new(outlet_in_use::new()),
        ],
        ..Default::default()
    }
}
