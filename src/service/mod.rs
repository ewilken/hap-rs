use characteristic;

pub mod accessory_information;
pub mod outlet;

pub trait ServiceT {
    fn get_characteristics(&self) -> Vec<&characteristic::CharacteristicT>;
}
