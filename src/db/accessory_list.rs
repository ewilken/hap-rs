use std::sync::{Arc, Mutex};
use std::ops::Deref;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use erased_serde;

use accessory::HapAccessory;
use characteristic::{HapCharacteristic, Perm};

use transport::http::Status;
use transport::http::handlers::characteristics::{WriteObject, WriteResponseObject};

#[derive(Clone)]
pub struct AccessoryList {
    accessories: Arc<Mutex<Vec<Box<AccessoryListTrait>>>>,
}

impl AccessoryList {
    pub fn write_characteristic(&self, write_object: WriteObject) -> WriteResponseObject {
        let mut result_object = WriteResponseObject {
            iid: write_object.iid,
            aid: write_object.aid,
            status: 0,
        };

        let mut a = self.accessories.lock().unwrap();
        'a: for accessory in a.iter_mut() {
            if accessory.get_id() == write_object.aid {
                for service in accessory.get_mut_services() {
                    for characteristic in service.get_mut_characteristics() {
                        if characteristic.get_id() == write_object.iid {
                            // TODO - permission checking
                            // let perms = characteristic.get_perms();
                            // if perms.contains(&Perm::PairedWrite) {}
                            if let Some(value) = write_object.value {
                                // TODO - handle error
                                characteristic.set_value(value).unwrap();
                            }
                            if let Some(ev) = write_object.ev {
                                characteristic.set_event_notifications(ev);
                            }
                            break 'a;
                        }
                    }
                }
            }
        }

        result_object
    }
}

pub trait AccessoryListTrait: HapAccessory + erased_serde::Serialize {}

impl<T: HapAccessory + erased_serde::Serialize> AccessoryListTrait for T {}

serialize_trait_object!(AccessoryListTrait);

impl Serialize for AccessoryList {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("AccessoryList", 1)?;
        let a = self.accessories.deref();
        state.serialize_field("accessories", &a)?;
        state.end()
    }
}

pub fn new(accessories: Vec<Box<AccessoryListTrait>>) -> AccessoryList {
    AccessoryList {
        accessories: Arc::new(Mutex::new(accessories))
    }
}
