use service;

pub struct Accessory {
    id: u64,
    accessory_type: Type,
    services: Vec<service::Service>,
    info: service::AccessoryInformation,
    id_count: u64,
}

struct Info {
    name: String,
    serial_number: String,
    manufacturer: String,
    model: String,
}

enum Type {
    Unknown,
    Other,
    Bridge,
    Fan,
    GarageDoorOpener,
    Lightbulb,
    DoorLock,
    Outlet,
    Switch,
    Thermostat,
    Sensor,
    SecuritySystem,
    Door,
    Window,
    WindowCovering,
    ProgrammableSwitch,
    IPCamera,
    VideoDoorbell,
    AirPurifier,
    Heater,
    AirConditioner,
    Humidifer,
    Dehumidifier,
}
