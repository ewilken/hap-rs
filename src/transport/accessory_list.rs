use std::sync::Arc;
use std::ops::Deref;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use erased_serde;

use accessory::HapAccessory;

#[derive(Clone)]
pub struct AccessoryList {
    accessories: Arc<Vec<Box<AccessoryListTrait>>>,
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
        accessories: Arc::new(accessories)
    }
}
