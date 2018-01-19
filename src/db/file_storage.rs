use std::fs;
use std::io::{Error, ErrorKind, Read, Write, BufReader, BufWriter};
use byteorder::{ByteOrder, BigEndian, ReadBytesExt, WriteBytesExt};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use uuid::Uuid;

use db::storage::Storage;

pub struct FileStorage {
    dir_path: PathBuf,
}

impl FileStorage {
    pub fn new(dir: &str) -> Result<FileStorage, Error> {
        let path = Path::new(dir).canonicalize()?;
        fs::create_dir_all(&path)?;

        let mut perms = fs::metadata(&path)?.permissions();
        perms.set_mode(0o777);
        fs::set_permissions(&path, perms)?;

        Ok(FileStorage {dir_path: path})
    }

    fn file_for_read(&self, file: &str) -> Result<fs::File, Error> {
        let file_path = self.path_to_file(file);
        let file = fs::OpenOptions::new()
            .read(true)
            .open(file_path)?;

        Ok(file)
    }

    fn file_for_write(&self, file: &str) -> Result<fs::File, Error> {
        let file_path = self.path_to_file(file);
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;

        Ok(file)
    }

    fn path_to_file(&self, file: &str) -> PathBuf {
        let mut file_path = self.dir_path.clone();
        file_path.set_file_name(file);

        file_path
    }
}

impl Storage for FileStorage {
    fn get_reader(&self, key: &str) -> Result<BufReader<fs::File>, Error> {
        let file = self.file_for_read(key)?;
        let mut reader = BufReader::new(file);

        Ok(reader)
    }

    fn get_writer(&self, key: &str) -> Result<BufWriter<fs::File>, Error> {
        let file = self.file_for_write(key)?;
        let mut writer = BufWriter::new(file);

        Ok(writer)
    }

    fn get_byte_vec(&self, key: &str) -> Result<Vec<u8>, Error> {
        let mut reader = self.get_reader(key)?;
        let mut value = Vec::new();
        reader.read_to_end(&mut value)?;

        Ok(value)
    }

    fn set_byte_vec(&self, key: &str, value: Vec<u8>) -> Result<(), Error> {
        let mut writer = self.get_writer(key)?;
        writer.write(&value)?;

        Ok(())
    }

    fn get_u64(&self, key: &str) -> Result<u64, Error> {
        let mut reader = self.get_reader(key)?;
        let mut value = reader.read_u64::<BigEndian>()?;

        Ok(value)
    }

    fn set_u64(&self, key: &str, value: u64) -> Result<(), Error> {
        let mut buf = Vec::new();
        BigEndian::write_u64(&mut buf, value);
        self.set_byte_vec(key, buf)?;

        Ok(())
    }

    fn get_uuid(&self, key: &str) -> Result<Uuid, Error> {
        let mut reader = self.get_reader(key)?;
        let mut buf = Vec::new();
        reader.read(&mut buf)?;
        let value = Uuid::from_bytes(&buf);
        match value {
            Ok(value) => Ok(value),
            _ => Err(Error::new(ErrorKind::Other, "there was a problem parsing the UUID")),
        }
    }

    fn set_uuid(&self, key: &str, value: Uuid) -> Result<(), Error> {
        let mut writer = self.get_writer(key)?;
        writer.write(value.as_bytes())?;

        Ok(())
    }

    fn delete(&self, key: &str) -> Result<(), Error> {
        let file_path = self.path_to_file(key);
        fs::remove_file(file_path)?;

        Ok(())
    }
}
