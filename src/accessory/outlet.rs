use service;
use accessory::{AccessoryT, Information};

#[derive(Default)]
pub struct Outlet {
    id: u64,

    pub accessory_information: service::accessory_information::AccessoryInformation,
    pub outlet: service::outlet::Outlet,
}

impl AccessoryT for Outlet {
    fn get_services(&self) -> Vec<&service::ServiceT> {
        vec![
            &self.accessory_information,
            &self.outlet,
        ]
    }

    // TODO - is unwrap the right thing to do here?
    fn set_information(&mut self, information: Information) {
        self.accessory_information.identify.set_value(information.identify).unwrap();
        self.accessory_information.manufacturer.set_value(information.manufacturer).unwrap();
        self.accessory_information.model.set_value(information.model).unwrap();
        self.accessory_information.name.set_value(information.name).unwrap();
        self.accessory_information.serial_number.set_value(information.serial_number).unwrap();
        self.accessory_information.firmware_revision.set_value(information.firmware_revision).unwrap();
    }
}

pub fn new(information: Information) -> Outlet {
    let mut outlet = Outlet {
        accessory_information: service::accessory_information::new(),
        outlet: service::outlet::new(),
        ..Default::default()
    };
    outlet.set_information(information);

    outlet
}
