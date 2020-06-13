use std::{
    env,
    ffi::OsStr,
    fs,
    io::{BufReader, BufWriter, Read, Write},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    str,
};

use async_trait::async_trait;
use tokio::task::spawn_blocking;
use uuid::Uuid;

use crate::{pairing::Pairing, storage::Storage, Config, Error, Result};

/// `FileStorage` is an implementor of the `Storage` trait that stores data to the file system.
#[derive(Debug)]
pub struct FileStorage {
    dir_path: PathBuf,
}

impl FileStorage {
    /// Creates a new `FileStorage`.
    pub async fn new<D: AsRef<OsStr> + ?Sized>(dir: &D) -> Result<Self> {
        let dir_path = Path::new(dir).to_path_buf();
        let dir_path = spawn_blocking(move || -> Result<PathBuf> {
            fs::create_dir_all(&dir_path)?;

            let mut perms = fs::metadata(&dir_path)?.permissions();
            perms.set_mode(0o777);
            fs::set_permissions(&dir_path, perms)?;

            Ok(dir_path)
        })
        .await??;

        Ok(FileStorage { dir_path })
    }

    /// Creates a new `FileStorage` with the current directory as storage path.
    pub async fn current_dir() -> Result<Self> {
        let current_dir =
            spawn_blocking(move || -> Result<PathBuf> { env::current_dir().map_err(Error::from) }).await??;
        let current_dir = current_dir.to_str().expect("couldn't stringify current_dir");
        let data_path = format!("{}/data", current_dir);

        Self::new(&data_path).await
    }

    fn path_to_file(&self, file: &str) -> PathBuf {
        let mut file_path = self.dir_path.clone();
        file_path.push(file);
        file_path
    }

    async fn get_reader(&self, file: &str) -> Result<BufReader<fs::File>> {
        let file_path = self.path_to_file(file);
        let reader = spawn_blocking(move || -> Result<BufReader<fs::File>> {
            let file = fs::OpenOptions::new().read(true).open(file_path)?;
            let reader = BufReader::new(file);

            Ok(reader)
        })
        .await??;

        Ok(reader)
    }

    async fn get_writer(&self, file: &str) -> Result<BufWriter<fs::File>> {
        let file_path = self.path_to_file(file);
        let writer = spawn_blocking(move || -> Result<BufWriter<fs::File>> {
            let file = fs::OpenOptions::new().write(true).create(true).open(file_path)?;
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
        let file_path = self.path_to_file(key);
        spawn_blocking(move || -> Result<()> {
            fs::remove_file(file_path)?;

            Ok(())
        })
        .await??;

        Ok(())
    }

    async fn keys_with_suffix(&self, suffix: &'static str) -> Result<Vec<String>> {
        let dir_path = self.dir_path.clone();
        let extension = Some(OsStr::new(suffix));
        let keys = spawn_blocking(move || -> Result<Vec<String>> {
            let mut keys = Vec::new();
            for entry in fs::read_dir(&dir_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension() == extension {
                    let key = path
                        .file_stem()
                        .ok_or(Error::from_str("invalid file name"))?
                        .to_os_string()
                        .into_string()
                        .or(Err(Error::from_str("invalid file name")))?;
                    keys.push(key);
                }
            }

            Ok(keys)
        })
        .await??;

        Ok(keys)
    }
}

#[async_trait]
impl Storage for FileStorage {
    async fn load_config(&self) -> Result<Config> {
        let config_bytes = self.read_bytes("config.json").await?;
        let config = serde_json::from_slice(&config_bytes)?;
        Ok(config)
    }

    async fn save_config(&mut self, config: &Config) -> Result<()> {
        let config_bytes = serde_json::to_vec(&config)?;
        self.write_bytes("config.json", config_bytes).await
    }

    async fn delete_config(&mut self) -> Result<()> {
        let key = format!("config.json");
        self.remove_file(&key).await
    }

    async fn load_pairing(&self, id: &Uuid) -> Result<Pairing> {
        let key = format!("{}.json", id.to_simple().to_string());
        let pairing_bytes = self.read_bytes(&key).await?;

        Pairing::from_bytes(&pairing_bytes)
    }

    async fn save_pairing(&mut self, pairing: &Pairing) -> Result<()> {
        let key = format!("{}.json", pairing.id.to_simple().to_string());
        let pairing_bytes = pairing.as_bytes()?;
        self.write_bytes(&key, pairing_bytes).await
    }

    async fn delete_pairing(&mut self, id: &Uuid) -> Result<()> {
        let key = format!("{}.json", id.to_simple().to_string());
        self.remove_file(&key).await
    }

    async fn list_pairings(&self) -> Result<Vec<Pairing>> {
        let mut pairings = Vec::new();
        for key in self.keys_with_suffix("json").await? {
            if &key != "device" {
                let pairing_bytes = self.read_bytes(&key).await?;
                let pairing = Pairing::from_bytes(&pairing_bytes)?;
                pairings.push(pairing);
            }
        }

        Ok(pairings)
    }

    async fn count_pairings(&self) -> Result<usize> {
        let mut count = 0;
        for key in self.keys_with_suffix("json").await? {
            if &key != "device" {
                count += 1;
            }
        }

        Ok(count)
    }
}
