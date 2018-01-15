pub enum FeatureFlag {
    Zero,
    MFICompliant,
}

impl FeatureFlag {
    pub fn as_u8(&self) -> u8 {
        match self {
            &FeatureFlag::Zero => 0,
            &FeatureFlag::MFICompliant => 1,
        }
    }
}

pub enum StatusFlag {
    Zero,
    NotPaired,
    WifiNotConfigured,
    ProblemDetected,
}

impl StatusFlag {
    pub fn as_u8(&self) -> u8 {
        match self {
            &StatusFlag::Zero => 0,
            &StatusFlag::NotPaired => 1,
            &StatusFlag::WifiNotConfigured => 2,
            &StatusFlag::ProblemDetected => 3,
        }
    }
}
