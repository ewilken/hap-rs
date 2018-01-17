use std::fs;
use std::io::{Error, Read, Write, BufReader, BufWriter};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

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
    fn get(&self, key: &str) -> Result<Vec<u8>, Error> {
        let file = self.file_for_read(key)?;
        let mut buf_reader = BufReader::new(file);

        let mut value = Vec::new();
        buf_reader.read_to_end(&mut value)?;

        Ok(value)
    }

    fn set(&self, key: &str, value: Vec<u8>) -> Result<(), Error> {
        let file = self.file_for_write(key)?;
        let mut buf_writer = BufWriter::new(file);

        buf_writer.write(&value)?;

        Ok(())
    }

    fn delete(&self, key: &str) -> Result<(), Error> {
        let file_path = self.path_to_file(key);
        fs::remove_file(file_path)?;

        Ok(())
    }
}
