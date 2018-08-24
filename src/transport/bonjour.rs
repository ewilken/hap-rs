/// Bonjour Feature Flag.
#[derive(Copy, Clone)]
pub enum FeatureFlag {
    Zero = 0,
    MfiCompliant = 1,
}

/// Bonjour Status Flag.
#[derive(Copy, Clone)]
pub enum StatusFlag {
    Zero = 0,
    NotPaired = 1,
    WifiNotConfigured = 2,
    ProblemDetected = 3,
}
