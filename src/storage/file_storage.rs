use std::{
    env,
    ffi::OsStr,
    fs,
    io::{BufReader, BufWriter, Read, Write},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    str,
};

use uuid::Uuid;

use crate::{pairing::Pairing, storage::Storage, Config, Error, Result};

/// `FileStorage` is an implementor of the `Storage` trait that stores data to the file system.
#[derive(Debug)]
pub struct FileStorage {
    dir_path: PathBuf,
}

impl FileStorage {
    /// Creates a new `FileStorage`.
    pub fn new<D: AsRef<OsStr> + ?Sized>(dir: &D) -> Result<Self> {
        let dir_path = Path::new(dir).to_path_buf();
        fs::create_dir_all(&dir_path)?;

        let mut perms = fs::metadata(&dir_path)?.permissions();
        perms.set_mode(0o777);
        fs::set_permissions(&dir_path, perms)?;
        Ok(FileStorage { dir_path })
    }

    /// Creates a new `FileStorage` with the current directory as storage path.
    pub fn current_dir() -> Result<Self> {
        let current_dir = env::current_dir()?;
        let current_dir = current_dir.to_str().expect("couldn't stringify current_dir");
        let data_path = format!("{}/data", current_dir);
        Self::new(&data_path)
    }

    fn path_to_file(&self, file: &str) -> PathBuf {
        let mut file_path = self.dir_path.clone();
        file_path.push(file);
        file_path
    }

    fn get_reader(&self, file: &str) -> Result<BufReader<fs::File>> {
        let file_path = self.path_to_file(file);
        let file = fs::OpenOptions::new().read(true).open(file_path)?;
        let reader = BufReader::new(file);
        Ok(reader)
    }

    fn get_writer(&self, file: &str) -> Result<BufWriter<fs::File>> {
        let file_path = self.path_to_file(file);
        let file = fs::OpenOptions::new().write(true).create(true).open(file_path)?;
        let writer = BufWriter::new(file);
        Ok(writer)
    }

    fn read_bytes(&self, key: &str) -> Result<Vec<u8>> {
        let mut reader = self.get_reader(key)?;
        let mut value = Vec::new();
        reader.read_to_end(&mut value)?;
        Ok(value)
    }

    fn write_bytes(&self, key: &str, value: Vec<u8>) -> Result<()> {
        let mut writer = self.get_writer(key)?;
        writer.write_all(&value)?;
        Ok(())
    }

    fn remove_file(&self, key: &str) -> Result<()> {
        let file_path = self.path_to_file(key);
        fs::remove_file(file_path)?;
        Ok(())
    }

    fn keys_with_suffix(&self, suffix: &str) -> Result<Vec<String>> {
        let extension = Some(OsStr::new(suffix));
        let mut keys = Vec::new();
        for entry in fs::read_dir(&self.dir_path)? {
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
    }
}

impl Storage for FileStorage {
    fn load_config(&self) -> Result<Config> {
        let config_bytes = self.read_bytes("config.json")?;
        let config = serde_json::from_slice(&config_bytes)?;
        Ok(config)
    }

    fn save_config(&mut self, config: &Config) -> Result<()> {
        let config_bytes = serde_json::to_vec(&config)?;
        self.write_bytes("config.json", config_bytes)?;
        Ok(())
    }

    fn delete_config(&mut self) -> Result<()> {
        let key = format!("config.json");
        self.remove_file(&key)
    }

    fn select_pairing(&self, id: &Uuid) -> Result<Pairing> {
        let pairing_bytes = self.read_bytes(&id.to_simple().to_string())?;
        Pairing::from_bytes(&pairing_bytes)
    }

    fn insert_pairing(&mut self, pairing: &Pairing) -> Result<()> {
        let pairing_bytes = pairing.as_bytes()?;
        self.write_bytes(&pairing.id.to_simple().to_string(), pairing_bytes)?;
        Ok(())
    }

    fn delete_pairing(&mut self, id: &Uuid) -> Result<()> {
        let key = format!("{}.json", id.to_simple().to_string());
        self.remove_file(&key)
    }

    fn list_pairings(&self) -> Result<Vec<Pairing>> {
        let mut pairings = Vec::new();
        for key in self.keys_with_suffix("json")? {
            if &key != "device" {
                let pairing_bytes = self.read_bytes(&key)?;
                let pairing = Pairing::from_bytes(&pairing_bytes)?;
                pairings.push(pairing);
            }
        }
        Ok(pairings)
    }

    fn count_pairings(&self) -> Result<usize> {
        let mut count = 0;
        for key in self.keys_with_suffix("json")? {
            if &key != "device" {
                count += 1;
            }
        }
        Ok(count)
    }
}
