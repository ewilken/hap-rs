use async_trait::async_trait;
use log::debug;
use std::{
    env,
    ffi::OsStr,
    fs,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
    str,
};
use tokio::task::spawn_blocking;
use uuid::Uuid;

use crate::{pairing::Pairing, storage::Storage, Config, Error, Result};

/// [`FileStorage`](FileStorage) is an implementor of the [`Storage`](Storage) trait that stores data to the file
/// system.
#[derive(Debug)]
pub struct FileStorage {
    dir_path: PathBuf,
}

impl FileStorage {
    /// Creates a new [`FileStorage`](FileStorage).
    pub async fn new<D: AsRef<OsStr> + ?Sized>(dir: &D) -> Result<Self> {
        let dir_path = Path::new(dir).to_path_buf();
        let dir_path = spawn_blocking(move || -> Result<PathBuf> {
            fs::create_dir_all(&dir_path)?;

            let dir_path_str = dir_path.to_str().expect("couldn't stringify current_dir");
            // create subdirectory for pairings
            fs::create_dir_all(&format!("{}/pairings", dir_path_str))?;
            // create subdirectory for custom byte storage
            fs::create_dir_all(&format!("{}/misc", dir_path_str))?;

            Ok(dir_path)
        })
        .await??;

        Ok(FileStorage { dir_path })
    }

    /// Creates a new [`FileStorage`](FileStorage) with the current directory as storage path.
    pub async fn current_dir() -> Result<Self> {
        let current_dir =
            spawn_blocking(move || -> Result<PathBuf> { env::current_dir().map_err(Error::from) }).await??;
        let current_dir = current_dir.to_str().expect("couldn't stringify current_dir");
        let data_path = format!("{}/data", current_dir);

        Self::new(&data_path).await
    }

    fn storage_path(&self, fd: &str) -> PathBuf {
        let mut fd_path = self.dir_path.clone();
        fd_path.push(fd);
        fd_path
    }

    async fn get_reader(&self, file: &str) -> Result<BufReader<fs::File>> {
        let file_path = self.storage_path(file);
        let reader = spawn_blocking(move || -> Result<BufReader<fs::File>> {
            let file = fs::OpenOptions::new().read(true).open(file_path)?;
            let reader = BufReader::new(file);

            Ok(reader)
        })
        .await??;

        Ok(reader)
    }

    async fn get_writer(&self, file: &str) -> Result<BufWriter<fs::File>> {
        let file_path = self.storage_path(file);
        let writer = spawn_blocking(move || -> Result<BufWriter<fs::File>> {
            let file = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_path)?;
            let writer = BufWriter::new(file);

            Ok(writer)
        })
        .await??;

        Ok(writer)
    }

    async fn read_bytes(&self, key: &str) -> Result<Vec<u8>> {
        let mut reader = self.get_reader(key).await?;
        let value = spawn_blocking(move || -> Result<Vec<u8>> {
            let mut value = Vec::new();
            reader.read_to_end(&mut value)?;

            Ok(value)
        })
        .await??;

        Ok(value)
    }

    async fn write_bytes(&self, key: &str, value: Vec<u8>) -> Result<()> {
        let mut writer = self.get_writer(key).await?;
        spawn_blocking(move || -> Result<()> {
            writer.write_all(&value)?;

            Ok(())
        })
        .await??;

        Ok(())
    }

    async fn remove_file(&self, key: &str) -> Result<()> {
        let file_path = self.storage_path(key);
        spawn_blocking(move || -> Result<()> {
            fs::remove_file(file_path)?;

            Ok(())
        })
        .await??;

        Ok(())
    }

    async fn list_files(&self, dir: PathBuf) -> Result<Vec<String>> {
        let file_names = spawn_blocking(move || -> Result<Vec<String>> {
            let mut file_names = Vec::new();
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                let file_name = path.into_os_string().into_string().or(Err(Error::Storage))?;
                file_names.push(file_name);
            }

            Ok(file_names)
        })
        .await??;

        Ok(file_names)
    }
}

#[async_trait]
impl Storage for FileStorage {
    async fn load_config(&self) -> Result<Config> {
        let config_bytes = self.read_bytes("config.json").await?;
        let config = serde_json::from_slice(&config_bytes)?;

        debug!("loaded Config: {:?}", &config);

        Ok(config)
    }

    async fn save_config(&mut self, config: &Config) -> Result<()> {
        let config_bytes = serde_json::to_vec(config)?;
        self.write_bytes("config.json", config_bytes).await
    }

    async fn delete_config(&mut self) -> Result<()> { self.remove_file("config.json").await }

    async fn load_aid_cache(&self) -> Result<Vec<u64>> {
        let aid_cache_bytes = self.read_bytes("aid_cache.json").await?;
        let aid_cache = serde_json::from_slice(&aid_cache_bytes)?;

        debug!("loaded AID cache: {:?}", &aid_cache);

        Ok(aid_cache)
    }

    async fn save_aid_cache(&mut self, aid_cache: &[u64]) -> Result<()> {
        let aid_cache_bytes = serde_json::to_vec(aid_cache)?;
        self.write_bytes("aid_cache.json", aid_cache_bytes).await
    }

    async fn delete_aid_cache(&mut self) -> Result<()> { self.remove_file("aid_cache.json").await }

    async fn load_pairing(&self, id: &Uuid) -> Result<Pairing> {
        let key = format!("pairings/{}.json", id.to_string());
        let pairing_bytes = self.read_bytes(&key).await?;

        let pairing = Pairing::from_bytes(&pairing_bytes)?;

        debug!("loaded Pairing: {:?}", &pairing);

        Ok(pairing)
    }

    async fn save_pairing(&mut self, pairing: &Pairing) -> Result<()> {
        let key = format!("pairings/{}.json", pairing.id.to_string());
        let pairing_bytes = pairing.as_bytes()?;
        self.write_bytes(&key, pairing_bytes).await
    }

    async fn delete_pairing(&mut self, id: &Uuid) -> Result<()> {
        let key = format!("pairings/{}.json", id.to_string());
        self.remove_file(&key).await
    }

    async fn list_pairings(&self) -> Result<Vec<Pairing>> {
        let pairing_dir = self.storage_path("pairings");

        let mut pairings = Vec::new();
        for key in self.list_files(pairing_dir).await? {
            let pairing_bytes = self.read_bytes(&key).await?;
            let pairing = Pairing::from_bytes(&pairing_bytes)?;
            pairings.push(pairing);
        }

        Ok(pairings)
    }

    async fn count_pairings(&self) -> Result<usize> {
        let pairing_dir = self.storage_path("pairings");

        let count = self.list_files(pairing_dir).await?.len();

        Ok(count)
    }

    async fn load_bytes(&self, key: &str) -> Result<Vec<u8>> {
        let bytes = self.read_bytes(&format!("misc/{}", key)).await?;

        Ok(bytes)
    }

    async fn save_bytes(&mut self, key: &str, value: &[u8]) -> Result<()> {
        self.write_bytes(&format!("misc/{}", key), value.to_vec()).await
    }

    async fn delete_bytes(&mut self, key: &str) -> Result<()> { self.remove_file(&format!("misc/{}", key)).await }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{pairing::Permissions, BonjourStatusFlag};

    /// Ensure we can write a [`Config`](Config), then a shorter one, without corrupting data.
    #[tokio::test]
    async fn test_config_storage() {
        let mut config = Default::default();

        let mut temp_dir = std::env::temp_dir();
        temp_dir.push("hap");

        let mut storage = FileStorage::new(&temp_dir).await.unwrap();

        // Config can't derive PartialEq
        let config_eq = |a: &Config, b: &Config| {
            assert_eq!(a.host, b.host);
            assert_eq!(a.port, b.port);
            assert_eq!(a.pin, b.pin);
            assert_eq!(a.name, b.name);
            assert_eq!(a.device_id, b.device_id);
            assert_eq!(a.configuration_number, b.configuration_number);
            assert_eq!(a.state_number, b.state_number);
            assert_eq!(a.category, b.category);
            assert_eq!(a.protocol_version, b.protocol_version);
            assert_eq!(a.status_flag, b.status_flag);
            assert_eq!(a.feature_flag, b.feature_flag);
            assert_eq!(a.max_peers, b.max_peers);
        };

        storage.save_config(&config).await.unwrap();

        // config should be correctly saved
        let saved_config = storage.load_config().await.unwrap();
        config_eq(&saved_config, &config);

        config.status_flag = BonjourStatusFlag::Zero;
        storage.save_config(&config).await.unwrap();

        // config should be correctly updated (Config can't derive PartialEq)
        let saved_config = storage.load_config().await.unwrap();
        config_eq(&saved_config, &config);

        storage.delete_config().await.unwrap();

        // config should be deleted
        let saved_config = storage.load_config().await;
        assert!(saved_config.is_err());
    }

    #[tokio::test]
    async fn test_aid_cache_storage() {
        let mut aid_cache = vec![1, 2, 3, 4];

        let mut temp_dir = std::env::temp_dir();
        temp_dir.push("hap");

        let mut storage = FileStorage::new(&temp_dir).await.unwrap();

        storage.save_aid_cache(&aid_cache).await.unwrap();

        // aid_cache should be correctly saved
        let saved_aid_cache = storage.load_aid_cache().await.unwrap();
        assert_eq!(saved_aid_cache, aid_cache);

        aid_cache.push(5);
        storage.save_aid_cache(&aid_cache).await.unwrap();

        // aid_cache should be correctly updated
        let saved_aid_cache = storage.load_aid_cache().await.unwrap();
        assert_eq!(saved_aid_cache, aid_cache);

        storage.delete_aid_cache().await.unwrap();

        // aid_cache should be deleted
        let saved_aid_cache = storage.load_aid_cache().await;
        assert!(saved_aid_cache.is_err());
    }

    /// Ensure we can correctly create, read, list and delete [`Pairing`](Pairing)s.
    #[tokio::test]
    async fn test_pairing_storage() {
        let pairing = Pairing {
            id: Uuid::parse_str("bc158b86-cabf-432d-aee4-422ef0e3f1d5").unwrap(),
            permissions: Permissions::Admin,
            public_key: [
                215, 90, 152, 1, 130, 177, 10, 183, 213, 75, 254, 211, 201, 100, 7, 58, 14, 225, 114, 243, 218, 166,
                35, 37, 175, 2, 26, 104, 247, 7, 81, 26,
            ],
        };

        let mut temp_dir = std::env::temp_dir();
        temp_dir.push("hap");

        let mut storage = FileStorage::new(&temp_dir).await.unwrap();

        // a fresh file storage should count 0 pairings, list an empty Vec, and error on a non-existent ID
        let pairing_count = storage.count_pairings().await.unwrap();
        assert_eq!(pairing_count, 0);

        let pairings = storage.list_pairings().await.unwrap();
        assert_eq!(pairings, vec![]);

        let saved_pairing = storage.load_pairing(&pairing.id).await;
        assert!(saved_pairing.is_err());

        // save a pairing
        storage.save_pairing(&pairing).await.unwrap();

        // after saving the first pairing, we should be able to count, load and list it
        let pairing_count = storage.count_pairings().await.unwrap();
        assert_eq!(pairing_count, 1);

        let pairings = storage.list_pairings().await.unwrap();
        assert_eq!(pairings.len(), 1);
        assert_eq!(&pairings[0], &pairing);

        let saved_pairing = storage.load_pairing(&pairing.id).await.unwrap();
        assert_eq!(&saved_pairing, &pairing);

        // delete the pairing
        storage.delete_pairing(&pairing.id).await.unwrap();

        // after deleting the previously saved pairing, it should count 0 again, list an empty
        // Vec and error on the now non-existent ID of the deleted pairing
        let pairing_count = storage.count_pairings().await.unwrap();
        assert_eq!(pairing_count, 0);

        let pairings = storage.list_pairings().await.unwrap();
        assert_eq!(pairings, vec![]);

        let saved_pairing = storage.load_pairing(&pairing.id).await;
        assert!(saved_pairing.is_err());
    }

    #[tokio::test]
    async fn test_byte_storage() {
        let mut bytes = vec![1, 2, 3, 4];

        let mut temp_dir = std::env::temp_dir();
        temp_dir.push("hap");

        let mut storage = FileStorage::new(&temp_dir).await.unwrap();

        storage.save_bytes("my_custom_bytes", &bytes).await.unwrap();

        // bytes should be correctly saved
        let saved_bytes = storage.load_bytes("my_custom_bytes").await.unwrap();
        assert_eq!(saved_bytes, bytes);

        bytes.push(5);
        storage.save_bytes("my_custom_bytes", &bytes).await.unwrap();

        // bytes should be correctly updated
        let saved_bytes = storage.load_bytes("my_custom_bytes").await.unwrap();
        assert_eq!(saved_bytes, bytes);

        storage.delete_bytes("my_custom_bytes").await.unwrap();

        // bytes should be deleted
        let saved_bytes = storage.load_bytes("my_custom_bytes").await;
        assert!(saved_bytes.is_err());
    }
}
