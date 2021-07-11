use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    accessory::{AccessoryInformation, HapAccessory},
    service::{
        accessory_information::AccessoryInformationService,
        lock_management::LockManagementService,
        lock_mechanism::LockMechanismService,
        HapService,
    },
    HapType,
    Result,
};

/// Lock Accessory.
#[derive(Default)]
pub struct LockAccessory {
    /// ID of the Lock Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformationService,
    /// Lock Mechanism Service.
    pub lock_mechanism: LockMechanismService,
    /// Lock Management Service.
    pub lock_management: LockManagementService,
}

impl LockAccessory {
    /// Creates a new Lock Accessory.
    pub fn new(id: u64, information: AccessoryInformation) -> Result<Self> {
        let accessory_information = information.to_service(1, id)?;

        let lock_mechanism_id = 2 + accessory_information.get_characteristics().len() as u64;
        let mut lock_mechanism = LockMechanismService::new(lock_mechanism_id, id);
        lock_mechanism.set_primary(true);

        let lock_management_id = 3 + lock_mechanism_id + lock_mechanism.get_characteristics().len() as u64;
        let mut lock_management = LockManagementService::new(lock_management_id, id);
        lock_management.set_primary(true);

        // TODO - figure out how to auto-set reasonable default values for tlv8 characteristics
        lock_management.logs = None;

        Ok(Self {
            id,
            accessory_information,
            lock_mechanism,
            lock_management,
        })
    }
}

impl HapAccessory for LockAccessory {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_service(&self, hap_type: HapType) -> Option<&dyn HapService> {
        for service in self.get_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_mut_service(&mut self, hap_type: HapType) -> Option<&mut dyn HapService> {
        for service in self.get_mut_services() {
            if service.get_type() == hap_type {
                return Some(service);
            }
        }
        None
    }

    fn get_services(&self) -> Vec<&dyn HapService> {
        vec![&self.accessory_information, &self.lock_mechanism, &self.lock_management]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
        vec![
            &mut self.accessory_information,
            &mut self.lock_mechanism,
            &mut self.lock_management,
        ]
    }
}

impl Serialize for LockAccessory {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapAccessory", 2)?;
        state.serialize_field("aid", &self.get_id())?;
        state.serialize_field("services", &self.get_services())?;
        state.end()
    }
}
