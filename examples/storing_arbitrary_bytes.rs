use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use hap::{
    accessory::{lightbulb::LightbulbAccessory, AccessoryCategory, AccessoryInformation},
    characteristic::AsyncCharacteristicCallbacks,
    futures::future::FutureExt,
    server::{IpServer, Server},
    storage::{FileStorage, Storage},
    Config,
    MacAddress,
    Pin,
    Result,
};

#[derive(Debug, Serialize, Deserialize)]
struct LightbulbState {
    pub power_state: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut lightbulb = LightbulbAccessory::new(1, AccessoryInformation {
        name: "Acme Stateful Lightbulb".into(),
        ..Default::default()
    })?;

    let mut storage = FileStorage::current_dir().await?;

    let config = match storage.load_config().await {
        Ok(mut config) => {
            config.redetermine_local_ip();
            storage.save_config(&config).await?;
            config
        },
        Err(_) => {
            let config = Config {
                pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3])?,
                name: "Acme Stateful Lightbulb".into(),
                device_id: MacAddress::from([10, 20, 30, 40, 50, 60]),
                category: AccessoryCategory::Lightbulb,
                ..Default::default()
            };
            storage.save_config(&config).await?;
            config
        },
    };

    let state = Arc::new(Mutex::new(match storage.load_bytes("state.json").await {
        Ok(state_bytes) => {
            let state = serde_json::from_slice(&state_bytes)?;
            state
        },
        Err(_) => {
            let state = LightbulbState { power_state: false };
            let state_bytes = serde_json::to_vec(&state)?;
            storage.save_bytes("state.json", &state_bytes).await?;
            state
        },
    }));

    let state_ = state.clone();
    lightbulb.lightbulb.power_state.on_read_async(Some(move || {
        let state = state_.clone();
        async move {
            let power_state = state.lock().await.power_state;

            println!("power_state characteristic read from state: {}", &power_state);

            // have the controller read the value from state
            Ok(Some(power_state))
        }
        .boxed()
    }));

    // TODO - creating another FileStorage instance on the same underlying directory isn't that elegant
    let state_storage = Arc::new(Mutex::new(FileStorage::current_dir().await?));
    let state_ = state.clone();
    let state_storage_ = state_storage.clone();
    lightbulb
        .lightbulb
        .power_state
        .on_update_async(Some(move |current_val: bool, new_val: bool| {
            let state = state_.clone();
            let storage = state_storage_.clone();
            async move {
                if current_val != new_val {
                    let mut s = state.lock().await;
                    s.power_state = new_val;

                    // persist the new value
                    let state_bytes = serde_json::to_vec(&*s)?;
                    storage.lock().await.save_bytes("state.json", &state_bytes).await?;
                }

                println!("power_state characteristic updated from {} to {}", current_val, new_val);

                Ok(())
            }
            .boxed()
        }));

    let server = IpServer::new(config, storage).await?;
    server.add_accessory(lightbulb).await?;

    let handle = server.run_handle();

    std::env::set_var("RUST_LOG", "hap=debug");
    env_logger::init();

    handle.await
}
