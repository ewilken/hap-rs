use crate::{
    accessory::{Accessory, HapAccessory, HapAccessoryService, Information},
    pointer,
    service::{accessory_information::AccessoryInformation, lock_management, lock_mechanism, HapService},
    Result,
};

/// Lock Accessory.
pub type Lock = Accessory<LockInner>;

/// Inner type of the Lock Accessory.
#[derive(Default)]
pub struct LockInner {
    /// ID of the Lock Accessory.
    id: u64,

    /// Accessory Information Service.
    pub accessory_information: AccessoryInformation,
    /// Lock Mechanism Service.
    pub lock_mechanism: lock_mechanism::LockMechanism,
    /// Lock Management Service.
    pub lock_management: lock_management::LockManagement,
}

impl HapAccessory for LockInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_services(&self) -> Vec<&dyn HapAccessoryService> {
        vec![&self.accessory_information, &self.lock_mechanism, &self.lock_management]
    }

    fn get_mut_services(&mut self) -> Vec<&mut dyn HapAccessoryService> {
        vec![
            &mut self.accessory_information,
            &mut self.lock_mechanism,
            &mut self.lock_management,
        ]
    }

    fn get_mut_information(&mut self) -> &mut AccessoryInformation { &mut self.accessory_information }

    fn init_iids(&mut self, accessory_id: u64, event_emitter: pointer::EventEmitter) -> Result<()> {
        let mut next_iid = 1;
        for service in self.get_mut_services() {
            service.set_id(next_iid);
            next_iid += 1;
            for characteristic in service.get_mut_characteristics() {
                characteristic.set_id(next_iid)?;
                characteristic.set_accessory_id(accessory_id)?;
                characteristic.set_event_emitter(Some(event_emitter.clone()))?;
                next_iid += 1;
            }
        }
        Ok(())
    }
}

/// Creates a new Lock Accessory.
pub fn new(information: Information) -> Result<Lock> {
    let mut lock_mechanism = lock_mechanism::new();
    lock_mechanism.set_primary(true);
    Ok(Lock::new(LockInner {
        accessory_information: information.to_service()?,
        lock_mechanism,
        lock_management: lock_management::new(),
        ..Default::default()
    }))
}
