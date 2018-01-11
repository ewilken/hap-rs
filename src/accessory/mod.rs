use service;

pub mod outlet;

pub trait AccessoryT {
    fn get_services(&self) -> Vec<&service::ServiceT>;
    fn set_information(&mut self, information: Information);
}

// TODO - maybe infere the types somehow from characteristic::Characteristic
#[derive(Default)]
pub struct Information {
    pub identify: bool,
    pub manufacturer: String,
    pub model: String,
    pub name: String,
    pub serial_number: String,
    pub firmware_revision: String,
}
