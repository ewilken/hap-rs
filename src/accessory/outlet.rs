use accessory::{Accessory, Information};
use service::{accessory_information, outlet};

pub type Outlet = Accessory;

pub fn new(information: Information) -> Outlet {
    let mut outlet = Outlet {
        services: vec![
            accessory_information::new(),
            outlet::new(),
        ],
        ..Default::default()
    };
    //outlet.set_information(information);
    outlet
}
