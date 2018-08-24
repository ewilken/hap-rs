use std::{rc::Rc, cell::RefCell, ops::Deref};

use serde::ser::{Serialize, Serializer, SerializeStruct};
use erased_serde;

use accessory::HapAccessory;
use characteristic::Perm;
use transport::http::{
    Status,
    server::EventSubscriptions,
    ReadResponseObject,
    WriteObject,
    WriteResponseObject,
};
use event::EmitterPtr;

use Error;

/// `AccessoryList` is a wrapper type holding an `Rc<RefCell>` with a `Vec` of boxed Accessories.
#[derive(Clone)]
pub struct AccessoryList {
    pub accessories: Rc<RefCell<Vec<Box<AccessoryListMember>>>>,
}

impl AccessoryList {
    /// Creates a new `AccessoryList`.
    pub fn new(accessories: Vec<Box<AccessoryListMember>>) -> AccessoryList {
        AccessoryList { accessories: Rc::new(RefCell::new(accessories)) }
    }

    pub(crate) fn init_aids(&mut self, event_emitter: EmitterPtr) -> Result<(), Error> {
        let mut next_aid = 1;
        for accessory in self.accessories.try_borrow_mut()?.iter_mut() {
            accessory.set_id(next_aid);
            accessory.init_iids(next_aid, event_emitter.clone())?;
            next_aid += 1;
        }
        Ok(())
    }

    pub(crate) fn read_characteristic(
        &self,
        aid: u64,
        iid: u64,
        meta: bool,
        perms: bool,
        hap_type: bool,
        ev: bool,
    ) -> Result<ReadResponseObject, Error> {
        let mut result_object = ReadResponseObject {
            iid,
            aid,
            hap_type: None,
            format: None,
            perms: None,
            ev: None,
            value: None,
            unit: None,
            max_value: None,
            min_value: None,
            step_value: None,
            max_len: None,
            status: Some(0),
        };

        'l: for accessory in self.accessories.try_borrow_mut()?.iter_mut() {
            if accessory.get_id() == aid {
                for service in accessory.get_mut_services() {
                    for characteristic in service.get_mut_characteristics() {
                        if characteristic.get_id()? == iid {
                            let characteristic_perms = characteristic.get_perms()?;
                            if characteristic_perms.contains(&Perm::PairedRead) {
                                result_object.value = Some(characteristic.get_value()?);
                                if meta {
                                    result_object.format = Some(characteristic.get_format()?);
                                    result_object.unit = characteristic.get_unit()?;
                                    result_object.max_value = characteristic.get_max_value()?;
                                    result_object.min_value = characteristic.get_min_value()?;
                                    result_object.step_value = characteristic.get_step_value()?;
                                    result_object.max_len = characteristic.get_max_len()?;
                                }
                                if perms {
                                    result_object.perms = Some(characteristic_perms);
                                }
                                if hap_type {
                                    result_object.hap_type = Some(characteristic.get_type()?);
                                }
                                if ev {
                                    result_object.ev = characteristic.get_event_notifications()?;
                                }
                            } else {
                                result_object.status = Some(Status::WriteOnlyCharacteristic as i32);
                            }
                            break 'l;
                        }
                    }
                }
            }
        }

        Ok(result_object)
    }

    pub(crate) fn write_characteristic(
        &self,
        write_object: WriteObject,
        event_subscriptions: &EventSubscriptions,
    ) -> Result<WriteResponseObject, Error> {
        let mut result_object = WriteResponseObject {
            aid: write_object.aid,
            iid: write_object.iid,
            status: 0,
        };

        let mut a = self.accessories.try_borrow_mut()?;
        'l: for accessory in a.iter_mut() {
            if accessory.get_id() == write_object.aid {
                for service in accessory.get_mut_services() {
                    for characteristic in service.get_mut_characteristics() {
                        if characteristic.get_id()? == write_object.iid {
                            let characteristic_perms = characteristic.get_perms()?;
                            if let Some(ev) = write_object.ev {
                                if characteristic_perms.contains(&Perm::Events) {
                                    characteristic.set_event_notifications(Some(ev))?;
                                    let subscription = (write_object.aid, write_object.iid);
                                    let mut es = event_subscriptions.try_borrow_mut()?;
                                    let pos = es.iter().position(|&s| s == subscription);
                                    match (ev, pos) {
                                        (true, None) => { es.push(subscription); },
                                        (false, Some(p)) => { es.remove(p); },
                                        _ => {},
                                    }
                                } else {
                                    result_object.status = Status::NotificationNotSupported as i32;
                                }
                            }
                            if let Some(value) = write_object.value {
                                if characteristic_perms.contains(&Perm::PairedWrite) {
                                    characteristic.set_value(value)?;
                                } else {
                                    result_object.status = Status::ReadOnlyCharacteristic as i32;
                                }
                            }
                            break 'l;
                        }
                    }
                }
            }
        }

        Ok(result_object)
    }
}

impl Serialize for AccessoryList {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("AccessoryList", 1)?;
        let a = self.accessories.deref();
        state.serialize_field("accessories", &a)?;
        state.end()
    }
}

/// `AccessoryListMember` is implemented by members of an `AccessoryList`.
pub trait AccessoryListMember: HapAccessory + erased_serde::Serialize {}

impl<T: HapAccessory + erased_serde::Serialize> AccessoryListMember for T {}

serialize_trait_object!(AccessoryListMember);
