#[derive(Copy, Clone)]
pub enum FeatureFlag {
    Zero = 0,
    MFICompliant = 1,
}

#[derive(Copy, Clone)]
pub enum StatusFlag {
    Zero = 0,
    NotPaired = 1,
    WifiNotConfigured = 2,
    ProblemDetected = 3,
}
