use serde::{Deserialize, Serialize};

/// Bonjour Feature Flag.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum BonjourFeatureFlag {
    Zero = 0,
    MfiCompliant = 1,
}

/// Bonjour Status Flag.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum BonjourStatusFlag {
    Zero = 0,
    NotPaired = 1,
    WifiNotConfigured = 2,
    ProblemDetected = 3,
}
