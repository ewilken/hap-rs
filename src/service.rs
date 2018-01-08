use characteristic;

pub struct Service {
    id: u64,
    service_type: Type,
    characteristics: Vec<characteristic::Characteristic>,
}

pub struct AccessoryInformation {

}

enum Type {
    AccessoryInformation,
    AirQualitySensor,
    BatteryService,
    BridgingState,
    CarbonDioxideSensor,
    CarbonMonoxideSensor,
    ContactSensor,
    Door,
    Fan,
    GarageDoorOpener,
    HumiditySensor,
    LeakSensor,
    LightSensor,
    Lightbulb,
    LockManagement,
    LockMechanism,
    MotionSensor,
    OccupancySensor,
    Outlet,
    SecuritySystem,
    SmokeSensor,
    StatefulProgrammableSwitch,
    StatelessProgrammableSwitch,
    Switch,
    TemperatureSensor,
    Thermostat,
    Window,
    WindowCovering,
}

impl Type {
    fn as_str(&self) -> &str {
        match self {
            &Type::AccessoryInformation => "3E",
            &Type::AirQualitySensor => "8D",
            &Type::BatteryService => "96",
            &Type::BridgingState => "62",
            &Type::CarbonDioxideSensor => "97",
            &Type::CarbonMonoxideSensor => "7F",
            &Type::ContactSensor => "80",
            &Type::Door => "81",
            &Type::Fan => "40",
            &Type::GarageDoorOpener => "41",
            &Type::HumiditySensor => "82",
            &Type::LeakSensor => "83",
            &Type::LightSensor => "84",
            &Type::Lightbulb => "43",
            &Type::LockManagement => "44",
            &Type::LockMechanism => "45",
            &Type::MotionSensor => "85",
            &Type::OccupancySensor => "86",
            &Type::Outlet => "47",
            &Type::SecuritySystem => "7E",
            &Type::SmokeSensor => "87",
            &Type::StatefulProgrammableSwitch => "88",
            &Type::StatelessProgrammableSwitch => "89",
            &Type::Switch => "49",
            &Type::TemperatureSensor => "8A",
            &Type::Thermostat => "4A",
            &Type::Window => "8B",
            &Type::WindowCovering => "8C",
        }
    }
}
