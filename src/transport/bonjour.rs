use serde::{Deserialize, Serialize};

/// Bonjour Feature Flag.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BonjourFeatureFlag {
    Zero = 0,
    SupportsHardwareAuthentication = 0x01,
    SupportsSoftwareAuthentication = 0x02,
}

/// Bonjour Status Flag.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BonjourStatusFlag {
    Zero = 0,
    /// Accessory has not been paired with any controllers.
    NotPaired = 0x01,
    /// Accessory has not been configured to join a Wi-Fi network.
    WifiNotConfigured = 0x02,
    /// A problem has been detected on the accessory.
    ProblemDetected = 0x04,
}
