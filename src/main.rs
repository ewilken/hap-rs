use std::sync::{Arc, Mutex};

extern crate hap;

use hap::{
    transport::{Transport, ip::IpTransport},
    accessory::{Category, Information, bridge, outlet, door, security_system, valve},
    characteristic::{Readable, Updatable},
    config::Config,
    hap_type::HapType,
};

pub struct VirtualOutlet {
    on: bool,
}

impl Readable<bool> for VirtualOutlet {
    fn on_read(&mut self, _: HapType) -> bool {
        println!("Outlet: On read.");
        self.on
    }
}

impl Updatable<bool> for VirtualOutlet {
    fn on_update(&mut self, _: HapType, old_val: &bool, new_val: &bool) {
        println!("Outlet: On updated from {} to {}.", old_val, new_val);
        if new_val != old_val { self.on = new_val.clone(); }
    }
}

pub struct VirtualDoor {
    current_position: u8,
    target_position: u8,
}

impl Readable<u8> for VirtualDoor {
    fn on_read(&mut self, hap_type: HapType) -> u8 {
        match hap_type {
            HapType::CurrentPosition => {
                println!("Door: Current position read.");
                self.current_position
            },
            HapType::TargetPosition => {
                println!("Door: Target position read.");
                self.target_position
            },
            // TODO - return optional?
            _ => 0,
        }
    }
}

impl Updatable<u8> for VirtualDoor {
    fn on_update(&mut self, hap_type: HapType, old_val: &u8, new_val: &u8) {
        match hap_type {
            HapType::CurrentPosition => {
                println!("Door: Current position updated from {} to {}.", old_val, new_val);
                if new_val != old_val { self.current_position = new_val.clone(); }
            },
            HapType::TargetPosition => {
                println!("Door: Target position updated from {} to {}.", old_val, new_val);
                if new_val != old_val { self.target_position = new_val.clone(); }
            },
            _ => {},
        }
    }
}

fn main() {
    let bridge = bridge::new(Information { name: "Bridge".into(), ..Default::default() });
    let mut outlet = outlet::new(Information { name: "Outlet".into(), ..Default::default() });

    // TODO - fix this
    let virtual_outlet = VirtualOutlet { on: false };
    let virtual_outlet_ptr = Arc::new(Mutex::new(Box::new(virtual_outlet)));
    outlet.inner.outlet.inner.on.set_readable(Some(virtual_outlet_ptr.clone()));
    outlet.inner.outlet.inner.on.set_updatable(Some(virtual_outlet_ptr.clone()));

    let mut door = door::new(Information { name: "Door".into(), ..Default::default() });

    let virtual_door = VirtualDoor { current_position: 0, target_position: 0 };
    let virtual_door_ptr = Arc::new(Mutex::new(Box::new(virtual_door)));
    door.inner.door.inner.current_position.set_readable(Some(virtual_door_ptr.clone()));
    door.inner.door.inner.current_position.set_updatable(Some(virtual_door_ptr.clone()));
    door.inner.door.inner.target_position.set_readable(Some(virtual_door_ptr.clone()));
    door.inner.door.inner.target_position.set_updatable(Some(virtual_door_ptr.clone()));

    let security_system = security_system::new(Information { name: "Security System".into(), ..Default::default() });
    let valve = valve::new(Information { name: "Valve".into(), ..Default::default() });

    let config = Config {
        name: "Korhal".into(),
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
