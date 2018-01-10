use service;

pub mod outlet;

pub trait AccessoryT {
    fn get_services(&self) -> Vec<&service::ServiceT>;
}
