// THIS FILE IS AUTO-GENERATED

use crate::{
    characteristic::{name, service_label_namespace, HapCharacteristic},
    service::{HapService, Service},
    HapType,
};

/// Service Label Service.
pub type ServiceLabel = Service<ServiceLabelInner>;

impl Default for ServiceLabel {
    fn default() -> ServiceLabel { new() }
}

/// Inner type of the Service Label Service.
#[derive(Default)]
pub struct ServiceLabelInner {
    /// ID of the Service Label Service.
    id: u64,
    /// `HapType` of the Service Label Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

    /// Service Label Namespace Characteristic.
    pub service_label_namespace: service_label_namespace::ServiceLabelNamespace,

    /// Name Characteristic.
    pub name: Option<name::Name>,
}

impl HapService for ServiceLabelInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_type(&self) -> HapType { self.hap_type }

    fn get_hidden(&self) -> bool { self.hidden }

    fn set_hidden(&mut self, hidden: bool) { self.hidden = hidden; }

    fn get_primary(&self) -> bool { self.primary }

    fn set_primary(&mut self, primary: bool) { self.primary = primary; }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![&self.service_label_namespace];
        if let Some(c) = &self.name {
            characteristics.push(c);
        }
        characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![&mut self.service_label_namespace];
        if let Some(c) = &mut self.name {
            characteristics.push(c);
        }
        characteristics
    }
}

/// Creates a new Service Label Service.
pub fn new() -> ServiceLabel {
    ServiceLabel::new(ServiceLabelInner {
        hap_type: HapType::ServiceLabel,
        service_label_namespace: service_label_namespace::new(),
        ..Default::default()
    })
}
