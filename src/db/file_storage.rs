use std::{
    fs,
    str,
    ffi::OsStr,
    io::{Read, Write, BufReader, BufWriter},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

use byteorder::{ByteOrder, BigEndian, ReadBytesExt};
use uuid::Uuid;

use db::storage::Storage;

use Error;

/// `FileStorage` is an implementor of the `Storage` trait that stores data to the file system.
pub struct FileStorage {
    dir_path: PathBuf,
}

impl FileStorage {
    /// Creates a new `FileStorage`.
    pub fn new(dir: &str) -> Result<FileStorage, Error> {
        let path = Path::new(dir).to_path_buf();
        fs::create_dir_all(&path)?;

        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o777);
        fs::set_permissions(&path, perms)?;
        Ok(FileStorage {dir_path: path})
    }

    /// Returns a readable `File` for the given file name.
    fn file_for_read(&self, file: &str) -> Result<fs::File, Error> {
        let file_path = self.path_to_file(file);
        let file = fs::OpenOptions::new()
            .read(true)
            .open(file_path)?;
        Ok(file)
    }

    /// Returns a writable `File` for the given file name.
    fn file_for_write(&self, file: &str) -> Result<fs::File, Error> {
        let file_path = self.path_to_file(file);
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;
        Ok(file)
    }

    /// Returns the full storage path for the given file name.
    fn path_to_file(&self, file: &str) -> PathBuf {
        let mut file_path = self.dir_path.clone();
        file_path.push(file);
        file_path
    }
}

impl Storage for FileStorage {
    fn get_reader(&self, key: &str) -> Result<BufReader<fs::File>, Error> {
        let file = self.file_for_read(key)?;
        let reader = BufReader::new(file);
        Ok(reader)
    }

    fn get_writer(&self, key: &str) -> Result<BufWriter<fs::File>, Error> {
        let file = self.file_for_write(key)?;
        let writer = BufWriter::new(file);
        Ok(writer)
    }

    fn get_bytes(&self, key: &str) -> Result<Vec<u8>, Error> {
        let mut reader = self.get_reader(key)?;
        let mut value = Vec::new();
        reader.read_to_end(&mut value)?;
        Ok(value)
    }

    fn set_bytes(&self, key: &str, value: Vec<u8>) -> Result<(), Error> {
        let mut writer = self.get_writer(key)?;
        writer.write_all(&value)?;
        Ok(())
    }

    fn get_u64(&self, key: &str) -> Result<u64, Error> {
        let mut reader = self.get_reader(key)?;
        let value = reader.read_u64::<BigEndian>()?;
        Ok(value)
    }

    fn set_u64(&self, key: &str, value: u64) -> Result<(), Error> {
        let mut buf = [0; 8];
        BigEndian::write_u64(&mut buf, value);
        self.set_bytes(key, buf.to_vec())?;
        Ok(())
    }

    fn get_uuid(&self, key: &str) -> Result<Uuid, Error> {
        let mut reader = self.get_reader(key)?;
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        match str::from_utf8(&buf) {
            Ok(uuid_str) => {
                match Uuid::parse_str(uuid_str) {
                    Ok(value) => Ok(value),
                    _ => Err(Error::new_io("couldn't parse UUID")),
                }
            },
            _ => Err(Error::new_io("couldn't read UUID")),
        }
    }

    fn set_uuid(&self, key: &str, value: Uuid) -> Result<(), Error> {
        let mut writer = self.get_writer(key)?;
        writer.write_all(value.hyphenated().to_string().as_bytes())?;
        Ok(())
    }

    fn keys_with_suffix(&self, suffix: &str) -> Result<Vec<String>, Error> {
        let extension = Some(OsStr::new(suffix));
        let mut keys = Vec::new();
        for entry in fs::read_dir(&self.dir_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension() == extension {
                let key = path.file_stem()
                    .ok_or(Error::new_io("invalid file name"))?
                    .to_os_string()
                    .into_string()
                    .or(Err(Error::new_io("invalid file name")))?;
                keys.push(key);
            }
        }
        Ok(keys)
    }

    fn delete(&self, key: &str) -> Result<(), Error> {
        let file_path = self.path_to_file(key);
        fs::remove_file(file_path)?;
        Ok(())
    }
}
