// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		current_transport::CurrentTransportCharacteristic,
		thread_control_point::ThreadControlPointCharacteristic,
		thread_node_capabilities::ThreadNodeCapabilitiesCharacteristic,
		thread_status::ThreadStatusCharacteristic,
		cca_energy_detect_threshold::CcaEnergyDetectThresholdCharacteristic,
		cca_signal_detect_threshold::CcaSignalDetectThresholdCharacteristic,
		event_retransmission_maximum::EventRetransmissionMaximumCharacteristic,
		event_transmission_counters::EventTransmissionCountersCharacteristic,
		mac_retransmission_maximum::MacRetransmissionMaximumCharacteristic,
		mac_transmission_counters::MacTransmissionCountersCharacteristic,
		receiver_sensitivity::ReceiverSensitivityCharacteristic,
		received_signal_strength_indication::ReceivedSignalStrengthIndicationCharacteristic,
		signal_to_noise_ratio::SignalToNoiseRatioCharacteristic,
		thread_openthread_version::ThreadOpenthreadVersionCharacteristic,
		transmit_power::TransmitPowerCharacteristic,
		maximum_transmit_power::MaximumTransmitPowerCharacteristic,
	},
    HapType,
};

/// Thread Transport service.
#[derive(Debug, Default)]
pub struct ThreadTransportService {
    /// Instance ID of the Thread Transport service.
    id: u64,
    /// [`HapType`](HapType) of the Thread Transport service.
    hap_type: HapType,
    /// When set to true, this service is not visible to user.
    hidden: bool,
    /// When set to true, this is the primary service on the accessory.
    primary: bool,
    /// An array of numbers containing the instance IDs of the services that this service links to.
    linked_services: Vec<u64>,

	/// Current Transport characteristic (required).
	pub current_transport: CurrentTransportCharacteristic,
	/// Thread Control Point characteristic (required).
	pub thread_control_point: ThreadControlPointCharacteristic,
	/// Thread Node Capabilities characteristic (required).
	pub thread_node_capabilities: ThreadNodeCapabilitiesCharacteristic,
	/// Thread Status characteristic (required).
	pub thread_status: ThreadStatusCharacteristic,

	/// CCA Energy Detect Threshold characteristic (optional).
	pub cca_energy_detect_threshold: Option<CcaEnergyDetectThresholdCharacteristic>,
	/// CCA Signal Detect Threshold characteristic (optional).
	pub cca_signal_detect_threshold: Option<CcaSignalDetectThresholdCharacteristic>,
	/// Event Retransmission Maximum characteristic (optional).
	pub event_retransmission_maximum: Option<EventRetransmissionMaximumCharacteristic>,
	/// Event Transmission Counters characteristic (optional).
	pub event_transmission_counters: Option<EventTransmissionCountersCharacteristic>,
	/// MAC Retransmission Maximum characteristic (optional).
	pub mac_retransmission_maximum: Option<MacRetransmissionMaximumCharacteristic>,
	/// MAC Transmission Counters characteristic (optional).
	pub mac_transmission_counters: Option<MacTransmissionCountersCharacteristic>,
	/// Receiver Sensitivity characteristic (optional).
	pub receiver_sensitivity: Option<ReceiverSensitivityCharacteristic>,
	/// Received Signal Strength Indication characteristic (optional).
	pub received_signal_strength_indication: Option<ReceivedSignalStrengthIndicationCharacteristic>,
	/// Signal-to-noise Ratio characteristic (optional).
	pub signal_to_noise_ratio: Option<SignalToNoiseRatioCharacteristic>,
	/// Thread OpenThread Version characteristic (optional).
	pub thread_openthread_version: Option<ThreadOpenthreadVersionCharacteristic>,
	/// Transmit Power characteristic (optional).
	pub transmit_power: Option<TransmitPowerCharacteristic>,
	/// Maximum Transmit Power characteristic (optional).
	pub maximum_transmit_power: Option<MaximumTransmitPowerCharacteristic>,
}

impl ThreadTransportService {
    /// Creates a new Thread Transport service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::ThreadTransport,
			current_transport: CurrentTransportCharacteristic::new(id + 1 + 0, accessory_id),
			thread_control_point: ThreadControlPointCharacteristic::new(id + 1 + 1, accessory_id),
			thread_node_capabilities: ThreadNodeCapabilitiesCharacteristic::new(id + 1 + 2, accessory_id),
			thread_status: ThreadStatusCharacteristic::new(id + 1 + 3, accessory_id),
			cca_energy_detect_threshold: Some(CcaEnergyDetectThresholdCharacteristic::new(id + 1 + 0 + 4, accessory_id)),
			cca_signal_detect_threshold: Some(CcaSignalDetectThresholdCharacteristic::new(id + 1 + 1 + 4, accessory_id)),
			event_retransmission_maximum: Some(EventRetransmissionMaximumCharacteristic::new(id + 1 + 2 + 4, accessory_id)),
			event_transmission_counters: Some(EventTransmissionCountersCharacteristic::new(id + 1 + 3 + 4, accessory_id)),
			mac_retransmission_maximum: Some(MacRetransmissionMaximumCharacteristic::new(id + 1 + 4 + 4, accessory_id)),
			mac_transmission_counters: Some(MacTransmissionCountersCharacteristic::new(id + 1 + 5 + 4, accessory_id)),
			receiver_sensitivity: Some(ReceiverSensitivityCharacteristic::new(id + 1 + 6 + 4, accessory_id)),
			received_signal_strength_indication: Some(ReceivedSignalStrengthIndicationCharacteristic::new(id + 1 + 7 + 4, accessory_id)),
			signal_to_noise_ratio: Some(SignalToNoiseRatioCharacteristic::new(id + 1 + 8 + 4, accessory_id)),
			thread_openthread_version: Some(ThreadOpenthreadVersionCharacteristic::new(id + 1 + 9 + 4, accessory_id)),
			transmit_power: Some(TransmitPowerCharacteristic::new(id + 1 + 10 + 4, accessory_id)),
			maximum_transmit_power: Some(MaximumTransmitPowerCharacteristic::new(id + 1 + 11 + 4, accessory_id)),
			..Default::default()
        }
    }
}

impl HapService for ThreadTransportService {
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
			&self.current_transport,
			&self.thread_control_point,
			&self.thread_node_capabilities,
			&self.thread_status,
		];
		if let Some(c) = &self.cca_energy_detect_threshold {
		    characteristics.push(c);
		}
		if let Some(c) = &self.cca_signal_detect_threshold {
		    characteristics.push(c);
		}
		if let Some(c) = &self.event_retransmission_maximum {
		    characteristics.push(c);
		}
		if let Some(c) = &self.event_transmission_counters {
		    characteristics.push(c);
		}
		if let Some(c) = &self.mac_retransmission_maximum {
		    characteristics.push(c);
		}
		if let Some(c) = &self.mac_transmission_counters {
		    characteristics.push(c);
		}
		if let Some(c) = &self.receiver_sensitivity {
		    characteristics.push(c);
		}
		if let Some(c) = &self.received_signal_strength_indication {
		    characteristics.push(c);
		}
		if let Some(c) = &self.signal_to_noise_ratio {
		    characteristics.push(c);
		}
		if let Some(c) = &self.thread_openthread_version {
		    characteristics.push(c);
		}
		if let Some(c) = &self.transmit_power {
		    characteristics.push(c);
		}
		if let Some(c) = &self.maximum_transmit_power {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        #[allow(unused_mut)]
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.current_transport,
			&mut self.thread_control_point,
			&mut self.thread_node_capabilities,
			&mut self.thread_status,
		];
		if let Some(c) = &mut self.cca_energy_detect_threshold {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.cca_signal_detect_threshold {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.event_retransmission_maximum {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.event_transmission_counters {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.mac_retransmission_maximum {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.mac_transmission_counters {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.receiver_sensitivity {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.received_signal_strength_indication {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.signal_to_noise_ratio {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.thread_openthread_version {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.transmit_power {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.maximum_transmit_power {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for ThreadTransportService {
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