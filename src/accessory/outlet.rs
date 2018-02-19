use serde_json;

use service::{self, HapService, accessory_information, outlet};
use accessory::{HapAccessory, Information};

#[derive(Default)]
pub struct Outlet {
    id: u64,

    pub accessory_information: accessory_information::AccessoryInformation,
    pub outlet: outlet::Outlet,
}

impl HapAccessory for Outlet {
    fn get_id(&self) -> &u64 {
        &self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&HapService> {
        vec![
            &self.accessory_information,
            &self.outlet,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut HapService> {
        vec![
            &mut self.accessory_information,
            &mut self.outlet,
        ]
    }

    fn set_information(&mut self, information: Information) {
        self.accessory_information.identify.set_value(information.identify).unwrap();
        self.accessory_information.manufacturer.set_value(information.manufacturer).unwrap();
        self.accessory_information.model.set_value(information.model).unwrap();
        self.accessory_information.name.set_value(information.name).unwrap();
        self.accessory_information.serial_number.set_value(information.serial_number).unwrap();
        self.accessory_information.firmware_revision.set_value(information.firmware_revision).unwrap();
    }

    fn to_json(&self) -> serde_json::Value {
        let services: Vec<serde_json::Value> = self.get_services().iter().map(|s| s.to_json()).collect();
        json!({
            "aid": self.get_id(),
            "services": services,
        })
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
