use service;
use accessory::AccessoryT;

#[derive(Default)]
pub struct Outlet {
    id: u64,

    information: service::accessory_information::AccessoryInformation,
    outlet: service::outlet::Outlet,
}

impl AccessoryT for Outlet {
    fn get_services(&self) -> Vec<&service::ServiceT> {
        vec![&self.information, &self.outlet]
    }
}

pub fn new() -> Outlet {
    Outlet {
        information: service::accessory_information::new(),
        outlet: service::outlet::new(),
        ..Default::default()
    }
}
