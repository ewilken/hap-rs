use std::{rc::Rc, cell::RefCell};

extern crate hap;

use hap::{
    transport::{Transport, IpTransport},
    accessory::{Category, Information, bridge, outlet, door, security_system, valve},
    characteristic::{Characteristic, Readable, Updatable},
    Config,
    HapType,
};

pub struct VirtualOutletInner {
    on: bool,
}

#[derive(Clone)]
pub struct VirtualOutlet {
    inner: Rc<RefCell<VirtualOutletInner>>
}

impl VirtualOutlet {
    pub fn new(inner: VirtualOutletInner) -> VirtualOutlet {
        VirtualOutlet { inner: Rc::new(RefCell::new(inner)) }
    }
}

impl Readable<bool> for VirtualOutlet {
    fn on_read(&mut self, _: HapType) -> Option<bool> {
        println!("Outlet: On read.");
        Some(self.inner.borrow().on)
    }
}

impl Updatable<bool> for VirtualOutlet {
    fn on_update(&mut self, old_val: &bool, new_val: &bool, _: HapType) {
        println!("Outlet: On updated from {} to {}.", old_val, new_val);
        if new_val != old_val { self.inner.borrow_mut().on = new_val.clone(); }
    }
}

pub struct VirtualDoorInner {
    current_position: u8,
    target_position: u8,
}

#[derive(Clone)]
pub struct VirtualDoor {
    inner: Rc<RefCell<VirtualDoorInner>>,
    current_position: Characteristic<u8>,
}

impl VirtualDoor {
    pub fn new(inner: VirtualDoorInner, current_position: Characteristic<u8>) -> VirtualDoor {
        VirtualDoor { inner: Rc::new(RefCell::new(inner)), current_position }
    }
}

impl Readable<u8> for VirtualDoor {
    fn on_read(&mut self, hap_type: HapType) -> Option<u8> {
        match hap_type {
            HapType::CurrentPosition => {
                println!("Door: Current position read.");
                Some(self.inner.borrow().current_position)
            },
            HapType::TargetPosition => {
                println!("Door: Target position read.");
                Some(self.inner.borrow().target_position)
            },
            _ => None,
        }
    }
}

impl Updatable<u8> for VirtualDoor {
    fn on_update(&mut self, old_val: &u8, new_val: &u8, hap_type: HapType) {
        match hap_type {
            HapType::CurrentPosition => {
                println!("Door: Current position updated from {} to {}.", old_val, new_val);
                if new_val != old_val {
                    self.inner.borrow_mut().current_position = new_val.clone();
                }
            },
            HapType::TargetPosition => {
                println!("Door: Target position updated from {} to {}.", old_val, new_val);
                if new_val != old_val {
                    {
                        let mut inner = self.inner.borrow_mut();
                        inner.target_position = new_val.clone();
                        inner.current_position = new_val.clone();
                    }
                    self.current_position.set_value(*new_val).expect("couldn't set value");
                }
            },
            _ => {},
        }
    }
}

fn main() {
    let bridge = bridge::new(Information {
        name: "Bridge".into(),
        ..Default::default()
    }).unwrap();

    let mut outlet = outlet::new(Information {
        name: "Outlet".into(),
        ..Default::default()
    }).unwrap();
    let virtual_outlet = VirtualOutlet::new(VirtualOutletInner { on: false });
    outlet.inner.outlet.inner.on.set_readable(virtual_outlet.clone()).unwrap();
    outlet.inner.outlet.inner.on.set_updatable(virtual_outlet).unwrap();

    let mut door = door::new(Information {
        name: "Door".into(),
        ..Default::default()
    }).unwrap();
    let virtual_door = VirtualDoor::new(
        VirtualDoorInner { current_position: 0, target_position: 0 },
        door.inner.door.inner.current_position.clone(),
    );
    door.inner.door.inner.current_position.set_readable(virtual_door.clone()).unwrap();
    door.inner.door.inner.current_position.set_updatable(virtual_door.clone()).unwrap();
    door.inner.door.inner.target_position.set_readable(virtual_door.clone()).unwrap();
    door.inner.door.inner.target_position.set_updatable(virtual_door).unwrap();

    let security_system = security_system::new(Information {
        name: "Security System".into(),
        ..Default::default()
    }).unwrap();

    let valve = valve::new(Information {
        name: "Valve".into(),
        ..Default::default()
    }).unwrap();

    let config = Config {
        name: "Acme Bridge".into(),
        category: Category::Bridge,
        ..Default::default()
    };
    let mut ip_transport = IpTransport::new(config, vec![
        Box::new(bridge),
        Box::new(outlet),
        Box::new(door),
        Box::new(security_system),
        Box::new(valve),
    ]).unwrap();

    ip_transport.start().unwrap();
}
