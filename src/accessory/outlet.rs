use accessory::{HapAccessory, HapAccessoryService, Accessory, Information};
use service::{accessory_information::{self, AccessoryInformation}, outlet};

pub type Outlet = Accessory<OutletInner>;

#[derive(Default)]
pub struct OutletInner {
    id: u64,

    pub accessory_information: AccessoryInformation,
    pub outlet: outlet::Outlet,
}

impl HapAccessory for OutletInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_services(&self) -> Vec<&HapAccessoryService> {
        vec![
            &self.accessory_information,
            &self.outlet,
        ]
    }

    fn get_mut_services(&mut self) -> Vec<&mut HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.outlet,
        ]
    }

    fn get_mut_information(&mut self) -> &mut AccessoryInformation {
        &mut self.accessory_information
    }
}

pub fn new(information: Information) -> Outlet {
    Outlet::new(OutletInner {
        accessory_information: accessory_information::new(Some(information)),
        outlet: outlet::new(),
        ..Default::default()
    })
}
