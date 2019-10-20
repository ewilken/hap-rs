// THIS FILE IS AUTO-GENERATED

use crate::{
    characteristic::{
        administrator_only_access,
        audio_feedback,
        current_door_state,
        lock_control_point,
        lock_last_known_action,
        lock_management_auto_security_timeout,
        logs,
        motion_detected,
        name,
        version,
        HapCharacteristic,
    },
    service::{HapService, Service},
    HapType,
};

/// Lock Management Service.
pub type LockManagement = Service<LockManagementInner>;

impl Default for LockManagement {
    fn default() -> LockManagement { new() }
}

/// Inner type of the Lock Management Service.
#[derive(Default)]
pub struct LockManagementInner {
    /// ID of the Lock Management Service.
    id: u64,
    /// `HapType` of the Lock Management Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

    /// Lock Control Point Characteristic.
    pub lock_control_point: lock_control_point::LockControlPoint,
    /// Version Characteristic.
    pub version: version::Version,

    /// Logs Characteristic.
    pub logs: Option<logs::Logs>,
    /// Audio Feedback Characteristic.
    pub audio_feedback: Option<audio_feedback::AudioFeedback>,
    /// Lock Management Auto Security Timeout Characteristic.
    pub lock_management_auto_security_timeout:
        Option<lock_management_auto_security_timeout::LockManagementAutoSecurityTimeout>,
    /// Administrator Only Access Characteristic.
    pub administrator_only_access: Option<administrator_only_access::AdministratorOnlyAccess>,
    /// Lock Last Known Action Characteristic.
    pub lock_last_known_action: Option<lock_last_known_action::LockLastKnownAction>,
    /// Current Door State Characteristic.
    pub current_door_state: Option<current_door_state::CurrentDoorState>,
    /// Motion Detected Characteristic.
    pub motion_detected: Option<motion_detected::MotionDetected>,
    /// Name Characteristic.
    pub name: Option<name::Name>,
}

impl HapService for LockManagementInner {
    fn get_id(&self) -> u64 { self.id }

    fn set_id(&mut self, id: u64) { self.id = id; }

    fn get_type(&self) -> HapType { self.hap_type }

    fn get_hidden(&self) -> bool { self.hidden }

    fn set_hidden(&mut self, hidden: bool) { self.hidden = hidden; }

    fn get_primary(&self) -> bool { self.primary }

    fn set_primary(&mut self, primary: bool) { self.primary = primary; }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![&self.lock_control_point, &self.version];
        if let Some(c) = &self.logs {
            characteristics.push(c);
        }
        if let Some(c) = &self.audio_feedback {
            characteristics.push(c);
        }
        if let Some(c) = &self.lock_management_auto_security_timeout {
            characteristics.push(c);
        }
        if let Some(c) = &self.administrator_only_access {
            characteristics.push(c);
        }
        if let Some(c) = &self.lock_last_known_action {
            characteristics.push(c);
        }
        if let Some(c) = &self.current_door_state {
            characteristics.push(c);
        }
        if let Some(c) = &self.motion_detected {
            characteristics.push(c);
        }
        if let Some(c) = &self.name {
            characteristics.push(c);
        }
        characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> =
            vec![&mut self.lock_control_point, &mut self.version];
        if let Some(c) = &mut self.logs {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.audio_feedback {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.lock_management_auto_security_timeout {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.administrator_only_access {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.lock_last_known_action {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.current_door_state {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.motion_detected {
            characteristics.push(c);
        }
        if let Some(c) = &mut self.name {
            characteristics.push(c);
        }
        characteristics
    }
}

/// Creates a new Lock Management Service.
pub fn new() -> LockManagement {
    LockManagement::new(LockManagementInner {
        hap_type: HapType::LockManagement,
        lock_control_point: lock_control_point::new(),
        version: version::new(),
        ..Default::default()
    })
}
