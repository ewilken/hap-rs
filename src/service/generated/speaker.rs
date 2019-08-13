// THIS FILE IS AUTO-GENERATED

use crate::{
    service::{HapService, Service},
    characteristic::{
        HapCharacteristic,
		mute,
		name,
		volume,
	},
    HapType,
};

/// Speaker Service.
pub type Speaker = Service<SpeakerInner>;

impl Default for Speaker {
    fn default() -> Speaker { new() }
}

/// Inner type of the Speaker Service.
#[derive(Default)]
pub struct SpeakerInner {
    /// ID of the Speaker Service.
    id: u64,
    /// `HapType` of the Speaker Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Mute Characteristic.
	pub mute: mute::Mute,

	/// Name Characteristic.
	pub name: Option<name::Name>,
	/// Volume Characteristic.
	pub volume: Option<volume::Volume>,
}

impl HapService for SpeakerInner {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn set_id(&mut self, id: u64) {
        self.id = id;
    }

    fn get_type(&self) -> HapType {
        self.hap_type
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

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
			&self.mute,
		];
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &self.volume {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.mute,
		];
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.volume {
		    characteristics.push(c);
		}
		characteristics
    }
}

/// Creates a new Speaker Service.
pub fn new() -> Speaker {
    Speaker::new(SpeakerInner {
        hap_type: HapType::Speaker,
		mute: mute::new(),
		..Default::default()
    })
}
