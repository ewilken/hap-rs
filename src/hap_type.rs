// THIS FILE IS AUTO-GENERATED

use serde::ser::{Serialize, Serializer};

/// HAP Service and Characteristic type.
#[derive(Copy, Clone, Debug)]
pub enum HapType {
    Unknown,
    AccessoryFlags,
    Active,
    ActiveIdentifier,
    AdministratorOnlyAccess,
    AirParticulateDensity,
    AirParticulateSize,
    AirQuality,
    AudioFeedback,
    BatteryLevel,
    Brightness,
    CarbonDioxideDetected,
    CarbonDioxideLevel,
    CarbonDioxidePeakLevel,
    CarbonMonoxideDetected,
    CarbonMonoxideLevel,
    CarbonMonoxidePeakLevel,
    ChargingState,
    ClosedCaptions,
    ConfiguredName,
    DisplayOrder,
    ColorTemperature,
    ContactSensorState,
    CoolingThresholdTemperature,
    CurrentAirPurifierState,
    CurrentAmbientLightLevel,
    CurrentDoorState,
    CurrentFanState,
    CurrentHeaterCoolerState,
    CurrentHeatingCoolingState,
    CurrentHorizontalTiltAngle,
    CurrentHumidifierDehumidifierState,
    CurrentMediaState,
    TargetMediaState,
    CurrentPosition,
    CurrentRelativeHumidity,
    CurrentSlatState,
    CurrentTemperature,
    CurrentTiltAngle,
    CurrentVerticalTiltAngle,
    DigitalZoom,
    FilterChangeIndication,
    FilterLifeLevel,
    FirmwareRevision,
    HardwareRevision,
    HeatingThresholdTemperature,
    HoldPosition,
    Hue,
    Identify,
    InputSourceType,
    InputDeviceType,
    Identifier,
    CurrentVisibilityState,
    TargetVisibilityState,
    ImageMirroring,
    ImageRotation,
    InUse,
    IsConfigured,
    LeakDetected,
    LockControlPoint,
    LockCurrentState,
    LockLastKnownAction,
    LockManagementAutoSecurityTimeout,
    LockPhysicalControls,
    LockTargetState,
    Logs,
    Manufacturer,
    Model,
    MotionDetected,
    Mute,
    Name,
    NightVision,
    NitrogenDioxideDensity,
    ObstructionDetected,
    OccupancyDetected,
    On,
    OpticalZoom,
    OutletInUse,
    OzoneDensity,
    PairSetup,
    PairVerify,
    PairingFeatures,
    PairingPairings,
    PM10Density,
    PM2_5Density,
    PositionState,
    PictureMode,
    PowerModeSelection,
    ProgramMode,
    ProgrammableSwitchEvent,
    RemoteKey,
    RelativeHumidityDehumidifierThreshold,
    RelativeHumidityHumidifierThreshold,
    RemainingDuration,
    ResetFilterIndication,
    RotationDirection,
    RotationSpeed,
    Saturation,
    SecuritySystemAlarmType,
    SecuritySystemCurrentState,
    SecuritySystemTargetState,
    SelectedRTPStreamConfiguration,
    SerialNumber,
    ServiceLabelIndex,
    ServiceLabelNamespace,
    SetDuration,
    SetupEndpoints,
    SlatType,
    SleepDiscoveryMode,
    SmokeDetected,
    StatusActive,
    StatusFault,
    StatusJammed,
    StatusLowBattery,
    StatusTampered,
    StreamingStatus,
    SulphurDioxideDensity,
    SupportedAudioStreamConfiguration,
    SupportedRTPConfiguration,
    SupportedVideoStreamConfiguration,
    SwingMode,
    TargetAirPurifierState,
    TargetAirQuality,
    TargetDoorState,
    TargetFanState,
    TargetHeaterCoolerState,
    TargetHeatingCoolingState,
    TargetHorizontalTiltAngle,
    TargetHumidifierDehumidifierState,
    TargetPosition,
    TargetRelativeHumidity,
    TargetSlatState,
    TargetTemperature,
    TargetTiltAngle,
    TargetVerticalTiltAngle,
    TemperatureDisplayUnits,
    ValveType,
    Version,
    VOCDensity,
    Volume,
    VolumeControlType,
    VolumeSelector,
    WaterLevel,
    AccessoryInformation,
    AirPurifier,
    AirQualitySensor,
    BatteryService,
    CameraRTPStreamManagement,
    CarbonDioxideSensor,
    CarbonMonoxideSensor,
    ContactSensor,
    Door,
    Doorbell,
    Fan,
    Fanv2,
    FilterMaintenance,
    Faucet,
    GarageDoorOpener,
    HeaterCooler,
    HumidifierDehumidifier,
    HumiditySensor,
    IrrigationSystem,
    LeakSensor,
    LightSensor,
    Lightbulb,
    LockManagement,
    LockMechanism,
    Microphone,
    MotionSensor,
    OccupancySensor,
    Outlet,
    SecuritySystem,
    ServiceLabel,
    Slat,
    SmokeSensor,
    Speaker,
    StatelessProgrammableSwitch,
    Switch,
    TemperatureSensor,
    Thermostat,
    Valve,
    Window,
    WindowCovering,
    Television,
    InputSource,
}

impl HapType {
    /// Converts a `HapType` to its corresponding shortened UUID string.
    pub fn to_string(self) -> String {
        match self {
            HapType::Unknown => "unknown".into(),
            HapType::AccessoryFlags => "A6".into(),
            HapType::Active => "B0".into(),
            HapType::ActiveIdentifier => "E7".into(),
            HapType::AdministratorOnlyAccess => "1".into(),
            HapType::AirParticulateDensity => "64".into(),
            HapType::AirParticulateSize => "65".into(),
            HapType::AirQuality => "95".into(),
            HapType::AudioFeedback => "5".into(),
            HapType::BatteryLevel => "68".into(),
            HapType::Brightness => "8".into(),
            HapType::CarbonDioxideDetected => "92".into(),
            HapType::CarbonDioxideLevel => "93".into(),
            HapType::CarbonDioxidePeakLevel => "94".into(),
            HapType::CarbonMonoxideDetected => "69".into(),
            HapType::CarbonMonoxideLevel => "90".into(),
            HapType::CarbonMonoxidePeakLevel => "91".into(),
            HapType::ChargingState => "8F".into(),
            HapType::ClosedCaptions => "DD".into(),
            HapType::ConfiguredName => "E3".into(),
            HapType::DisplayOrder => "136".into(),
            HapType::ColorTemperature => "CE".into(),
            HapType::ContactSensorState => "6A".into(),
            HapType::CoolingThresholdTemperature => "D".into(),
            HapType::CurrentAirPurifierState => "A9".into(),
            HapType::CurrentAmbientLightLevel => "6B".into(),
            HapType::CurrentDoorState => "E".into(),
            HapType::CurrentFanState => "AF".into(),
            HapType::CurrentHeaterCoolerState => "B1".into(),
            HapType::CurrentHeatingCoolingState => "F".into(),
            HapType::CurrentHorizontalTiltAngle => "6C".into(),
            HapType::CurrentHumidifierDehumidifierState => "B3".into(),
            HapType::CurrentMediaState => "E0".into(),
            HapType::TargetMediaState => "137".into(),
            HapType::CurrentPosition => "6D".into(),
            HapType::CurrentRelativeHumidity => "10".into(),
            HapType::CurrentSlatState => "AA".into(),
            HapType::CurrentTemperature => "11".into(),
            HapType::CurrentTiltAngle => "C1".into(),
            HapType::CurrentVerticalTiltAngle => "6E".into(),
            HapType::DigitalZoom => "11D".into(),
            HapType::FilterChangeIndication => "AC".into(),
            HapType::FilterLifeLevel => "AB".into(),
            HapType::FirmwareRevision => "52".into(),
            HapType::HardwareRevision => "53".into(),
            HapType::HeatingThresholdTemperature => "12".into(),
            HapType::HoldPosition => "6F".into(),
            HapType::Hue => "13".into(),
            HapType::Identify => "14".into(),
            HapType::InputSourceType => "DB".into(),
            HapType::InputDeviceType => "DC".into(),
            HapType::Identifier => "E6".into(),
            HapType::CurrentVisibilityState => "135".into(),
            HapType::TargetVisibilityState => "134".into(),
            HapType::ImageMirroring => "11F".into(),
            HapType::ImageRotation => "11E".into(),
            HapType::InUse => "D2".into(),
            HapType::IsConfigured => "D6".into(),
            HapType::LeakDetected => "70".into(),
            HapType::LockControlPoint => "19".into(),
            HapType::LockCurrentState => "1D".into(),
            HapType::LockLastKnownAction => "1C".into(),
            HapType::LockManagementAutoSecurityTimeout => "1A".into(),
            HapType::LockPhysicalControls => "A7".into(),
            HapType::LockTargetState => "1E".into(),
            HapType::Logs => "1F".into(),
            HapType::Manufacturer => "20".into(),
            HapType::Model => "21".into(),
            HapType::MotionDetected => "22".into(),
            HapType::Mute => "11A".into(),
            HapType::Name => "23".into(),
            HapType::NightVision => "11B".into(),
            HapType::NitrogenDioxideDensity => "C4".into(),
            HapType::ObstructionDetected => "24".into(),
            HapType::OccupancyDetected => "71".into(),
            HapType::On => "25".into(),
            HapType::OpticalZoom => "11C".into(),
            HapType::OutletInUse => "26".into(),
            HapType::OzoneDensity => "C3".into(),
            HapType::PairSetup => "4C".into(),
            HapType::PairVerify => "4E".into(),
            HapType::PairingFeatures => "4F".into(),
            HapType::PairingPairings => "50".into(),
            HapType::PM10Density => "C7".into(),
            HapType::PM2_5Density => "C6".into(),
            HapType::PositionState => "72".into(),
            HapType::PictureMode => "E2".into(),
            HapType::PowerModeSelection => "DF".into(),
            HapType::ProgramMode => "D1".into(),
            HapType::ProgrammableSwitchEvent => "73".into(),
            HapType::RemoteKey => "E1".into(),
            HapType::RelativeHumidityDehumidifierThreshold => "C9".into(),
            HapType::RelativeHumidityHumidifierThreshold => "CA".into(),
            HapType::RemainingDuration => "D4".into(),
            HapType::ResetFilterIndication => "AD".into(),
            HapType::RotationDirection => "28".into(),
            HapType::RotationSpeed => "29".into(),
            HapType::Saturation => "2F".into(),
            HapType::SecuritySystemAlarmType => "8E".into(),
            HapType::SecuritySystemCurrentState => "66".into(),
            HapType::SecuritySystemTargetState => "67".into(),
            HapType::SelectedRTPStreamConfiguration => "117".into(),
            HapType::SerialNumber => "30".into(),
            HapType::ServiceLabelIndex => "CB".into(),
            HapType::ServiceLabelNamespace => "CD".into(),
            HapType::SetDuration => "D3".into(),
            HapType::SetupEndpoints => "118".into(),
            HapType::SlatType => "C0".into(),
            HapType::SleepDiscoveryMode => "E8".into(),
            HapType::SmokeDetected => "76".into(),
            HapType::StatusActive => "75".into(),
            HapType::StatusFault => "77".into(),
            HapType::StatusJammed => "78".into(),
            HapType::StatusLowBattery => "79".into(),
            HapType::StatusTampered => "7A".into(),
            HapType::StreamingStatus => "120".into(),
            HapType::SulphurDioxideDensity => "C5".into(),
            HapType::SupportedAudioStreamConfiguration => "115".into(),
            HapType::SupportedRTPConfiguration => "116".into(),
            HapType::SupportedVideoStreamConfiguration => "114".into(),
            HapType::SwingMode => "B6".into(),
            HapType::TargetAirPurifierState => "A8".into(),
            HapType::TargetAirQuality => "AE".into(),
            HapType::TargetDoorState => "32".into(),
            HapType::TargetFanState => "BF".into(),
            HapType::TargetHeaterCoolerState => "B2".into(),
            HapType::TargetHeatingCoolingState => "33".into(),
            HapType::TargetHorizontalTiltAngle => "7B".into(),
            HapType::TargetHumidifierDehumidifierState => "B4".into(),
            HapType::TargetPosition => "7C".into(),
            HapType::TargetRelativeHumidity => "34".into(),
            HapType::TargetSlatState => "BE".into(),
            HapType::TargetTemperature => "35".into(),
            HapType::TargetTiltAngle => "C2".into(),
            HapType::TargetVerticalTiltAngle => "7D".into(),
            HapType::TemperatureDisplayUnits => "36".into(),
            HapType::ValveType => "D5".into(),
            HapType::Version => "37".into(),
            HapType::VOCDensity => "C8".into(),
            HapType::Volume => "119".into(),
            HapType::VolumeControlType => "E9".into(),
            HapType::VolumeSelector => "EA".into(),
            HapType::WaterLevel => "B5".into(),
            HapType::AccessoryInformation => "3E".into(),
            HapType::AirPurifier => "BB".into(),
            HapType::AirQualitySensor => "8D".into(),
            HapType::BatteryService => "96".into(),
            HapType::CameraRTPStreamManagement => "110".into(),
            HapType::CarbonDioxideSensor => "97".into(),
            HapType::CarbonMonoxideSensor => "7F".into(),
            HapType::ContactSensor => "80".into(),
            HapType::Door => "81".into(),
            HapType::Doorbell => "121".into(),
            HapType::Fan => "40".into(),
            HapType::Fanv2 => "B7".into(),
            HapType::FilterMaintenance => "BA".into(),
            HapType::Faucet => "D7".into(),
            HapType::GarageDoorOpener => "41".into(),
            HapType::HeaterCooler => "BC".into(),
            HapType::HumidifierDehumidifier => "BD".into(),
            HapType::HumiditySensor => "82".into(),
            HapType::IrrigationSystem => "CF".into(),
            HapType::LeakSensor => "83".into(),
            HapType::LightSensor => "84".into(),
            HapType::Lightbulb => "43".into(),
            HapType::LockManagement => "44".into(),
            HapType::LockMechanism => "45".into(),
            HapType::Microphone => "112".into(),
            HapType::MotionSensor => "85".into(),
            HapType::OccupancySensor => "86".into(),
            HapType::Outlet => "47".into(),
            HapType::SecuritySystem => "7E".into(),
            HapType::ServiceLabel => "CC".into(),
            HapType::Slat => "B9".into(),
            HapType::SmokeSensor => "87".into(),
            HapType::Speaker => "113".into(),
            HapType::StatelessProgrammableSwitch => "89".into(),
            HapType::Switch => "49".into(),
            HapType::TemperatureSensor => "8A".into(),
            HapType::Thermostat => "4A".into(),
            HapType::Valve => "D0".into(),
            HapType::Window => "8B".into(),
            HapType::WindowCovering => "8C".into(),
            HapType::Television => "D8".into(),
            HapType::InputSource => "D9".into(),
        }
    }
}

impl Default for HapType {
    fn default() -> HapType { HapType::Unknown }
}

impl Serialize for HapType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
