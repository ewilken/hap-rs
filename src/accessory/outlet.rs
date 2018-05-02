use accessory::{HapAccessory, HapAccessoryService, Accessory, Information};
use service::{accessory_information::{self, AccessoryInformation}, outlet};
use event::EmitterPtr;

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

    fn init_iids(&mut self, accessory_id: u64, event_emitter: EmitterPtr) {
        let mut next_iid = 1;
        for service in self.get_mut_services() {
            service.set_id(next_iid);
            next_iid += 1;
            for characteristic in service.get_mut_characteristics() {
                characteristic.set_id(next_iid);
                characteristic.set_accessory_id(accessory_id);
                characteristic.set_event_emitter(Some(event_emitter.clone()));
                next_iid += 1;
            }
        }
    }
}

pub fn new(information: Information) -> Outlet {
    Outlet::new(OutletInner {
        accessory_information: accessory_information::new(Some(information)),
        outlet: outlet::new(),
        ..Default::default()
    })
}
