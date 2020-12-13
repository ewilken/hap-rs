# HAP (HomeKit Accessory Protocol)

[![CI](https://github.com/ewilken/hap-rs/workflows/CI/badge.svg)](https://github.com/ewilken/hap-rs/actions?query=workflow%3ACI)
[![crates.io](https://img.shields.io/crates/v/hap.svg)](https://crates.io/crates/hap)
[![docs.rs](https://docs.rs/hap/badge.svg)](https://docs.rs/hap)
[![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/ewilken/hap-rs)

Rust implementation of the Apple HomeKit Accessory Protocol (HAP).

This crate supports all HomeKit services and characteristics currently implemented by Apple and provides the ability to create custom characteristics, services and accessories.

The HomeKit Accessory Protocol supports transports over IP and Bluetooth LE. Currently only the transport over IP is implemented in this crate. Accessories are exposed by the implemented HAP Accessory HTTP server and announced via built-in mDNS.

## HomeKit Data Model

The HAP defines HomeKit enabled devices as virtual `accessories` that are composed of `services` that are composed of `characteristics`.

Characteristics hold values of various data types as well as optional metadata like max/min values or units. Services group characteristics and represent features of the accessory. Every accessory consists of at least one `accessory information service` and any number of additional services. For example a custom ceiling fan accessory may consist of an `accessory information service`, a `fan service` and a `lightbulb service`.

```
Ceiling Fan Accessory
|
|-- Accessory Information Service
|   |-- Identify Characteristic
|   |-- Manufacturer Characteristic
|   |-- Model Characteristic
|   |-- Name Characteristic
|   |-- Serial Characteristic
|
|-- Fan Service
|   |-- On Characteristic
|   |-- Rotation Direction Characteristic
|   |-- Rotation Speed Characteristic
|
|-- Lightbulb Service
|   |-- On Characteristic
|   |-- Brightness Characteristic
|   |-- Hue Characteristic
|   |-- Saturation Characteristic
```

This crate provides a pre-built accessory for every service predefined by Apple in the HomeKit Accessory Simulator as well as others like Television. Custom characteristics and services can be created, assembled and used alongside the predefined ones.

For a full list of the predefined characteristics, services and accessories, see the [docs](https://docs.rs/hap/) or [Apple's official specification](https://developer.apple.com/homekit/).

## Usage Examples

### Creating a simple lightbulb accessory and starting the IP server

```rust
use hap::{
    accessory::{lightbulb::LightbulbAccessory, AccessoryCategory, AccessoryInformation},
    server::{IpServer, Server},
    storage::FileStorage,
    tokio,
    Config,
    MacAddress,
    Pin,
};

#[tokio::main]
async fn main() {
    let lightbulb = LightbulbAccessory::new(1, AccessoryInformation {
        name: "Acme Lightbulb".into(),
        ..Default::default()
    })
    .unwrap();

    let config = Config {
        pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3]).unwrap(),
        name: "Acme Lightbulb".into(),
        device_id: MacAddress::new([10, 20, 30, 40, 50, 60]),
        category: AccessoryCategory::Lightbulb,
        ..Default::default()
    };
    let storage = FileStorage::current_dir().await.unwrap();

    let mut server = IpServer::new(config, storage).unwrap();
    server.add_accessory(lightbulb).await.unwrap();

    let handle = server.run_handle();

    std::env::set_var("RUST_LOG", "hap=debug");
    env_logger::init();

    handle.await;
}
```

### Setting sync callbacks to react to remote value reads and updates

```rust
use hap::characteristic::CharacteristicCallbacks;

lightbulb.lightbulb.on.on_read(Some(|| {
    println!("on characteristic read");
    None
}));

lightbulb.lightbulb.on.on_update(Some(|current_val: &bool, new_val: &bool| {
    println!("on characteristic updated from {} to {}", current_val, new_val);
}));
```

### Setting async callbacks to react to remote value reads and updates

```rust
use hap::characteristic::AsyncCharacteristicCallbacks;

lightbulb.lightbulb.on.on_read_async(Some(|| {
    async {
        println!("on characteristic read (async)");
        None
    }
    .boxed()
}));

lightbulb.lightbulb.on.on_update_async(Some(|current_val: bool, new_val: bool| {
    async move {
        println!("on characteristic updated from {} to {} (async)", current_val, new_val);
    }
    .boxed()
}));
```

### Setting a characteristic value directly

```rust
use hap::{
    characteristic::HapCharacteristic,
    serde_json::Value,
};

lightbulb.lightbulb.on.set_value(Value::Bool(true)).await.unwrap();
```

### Interacting with accessories added to the server

`Server::add_accessory` returns a pointer to the accessory that can be used like this:

```rust
async {
    let accessory_ptr = server.add_accessory(accessory).await.unwrap();
}
```

Accessories behind the pointer are represented by the `HapAccessory` trait. The `HapAccessory::get_service` and `HapAccessory::get_mut_service` methods provide access to the services of the accessory, represented by the `HapService` trait. The `HapService::get_characteristic` and `HapService::get_mut_characteristic` methods provide access to the characteristics of the service, represented by the `HapCharacteristic` trait. All services and characteristics are identified by their `HapType`.

Accessing and changing the `on` characteristic of the `lightbulb` service of a `lightbulb` accessory would look like this:

```rust
use hap::{HapType, serde_json::Value};

async {
    let mut lightbulb_accessory = lightbulb_ptr.lock().await;

    let lightbulb_service = lightbulb_accessory.get_mut_service(HapType::Lightbulb).unwrap();
    let on_characteristic = lightbulb_service.get_mut_characteristic(HapType::On).unwrap();

    on_characteristic.set_value(Value::Bool(true)).await.unwrap();
}
```

A full working example can be found [here](https://github.com/ewilken/hap-rs/blob/master/examples/setting_values_after_server_start.rs).

## TODOs

- [x] IP Transport
- [x] Default Accessories
- [x] Lock Accessory
- [x] Television Accessory
- [ ] Camera Streams
  - [ ] IP Camera Accessory
  - [ ] Video Doorbell Accessory
- [ ] BLE Transport

## Development

Codegen is handled by the `codegen` crate in the workspace. Generated files are checked in. To run the code generation, do:

    cargo run --package hap-codegen

## License

HAP is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
