// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		programmable_switch_event::ProgrammableSwitchEventCharacteristic,
		brightness::BrightnessCharacteristic,
		mute::MuteCharacteristic,
		name::NameCharacteristic,
		operating_state_response::OperatingStateResponseCharacteristic,
		volume::VolumeCharacteristic,
	},
    HapType,
};

/// Doorbell service.
#[derive(Debug, Default)]
pub struct DoorbellService {
    /// Instance ID of the Doorbell service.
    id: u64,
    /// [`HapType`](HapType) of the Doorbell service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

	/// Programmable Switch Event characteristic (required).
	pub programmable_switch_event: ProgrammableSwitchEventCharacteristic,
	/// Brightness characteristic (optional).
	pub brightness: Option<BrightnessCharacteristic>,
	/// Mute characteristic (optional).
	pub mute: Option<MuteCharacteristic>,
	/// Name characteristic (optional).
	pub name: Option<NameCharacteristic>,
	/// Operating State Response characteristic (optional).
	pub operating_state_response: Option<OperatingStateResponseCharacteristic>,
	/// Volume characteristic (optional).
	pub volume: Option<VolumeCharacteristic>,
}

impl DoorbellService {
    /// Creates a new Doorbell service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::Doorbell,
			programmable_switch_event: ProgrammableSwitchEventCharacteristic::new(id  + 1, accessory_id),
			brightness: Some(BrightnessCharacteristic::new(id + 1  + 1, accessory_id)),
			mute: Some(MuteCharacteristic::new(id + 1 + 1  + 1, accessory_id)),
			name: Some(NameCharacteristic::new(id + 1 + 2  + 1, accessory_id)),
			operating_state_response: Some(OperatingStateResponseCharacteristic::new(id + 1 + 3  + 1, accessory_id)),
			volume: Some(VolumeCharacteristic::new(id + 1 + 4  + 1, accessory_id)),
			..Default::default()
        }
    }
}

impl HapService for DoorbellService {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn set_type(&mut self, hap_type: HapType) {
        self.hap_type = hap_type;
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    fn get_linked_services(&self) -> Vec<u64> {
        self.linked_services.clone()
    }

    fn set_linked_services(&mut self, linked_services: Vec<u64>) {
        self.linked_services = linked_services;
    }

    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
        for characteristic in self.get_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
        for characteristic in self.get_mut_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
			&self.programmable_switch_event,
		];
		if let Some(c) = &self.brightness {
		    characteristics.push(c);
		}
		if let Some(c) = &self.mute {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &self.operating_state_response {
		    characteristics.push(c);
		}
		if let Some(c) = &self.volume {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.programmable_switch_event,
		];
		if let Some(c) = &mut self.brightness {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.mute {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.operating_state_response {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.volume {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for DoorbellService {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}
