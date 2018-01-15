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

// TODO - rethink whether to use String or &str as key
impl Storage for FileStorage {
    fn get(&self, key: String) -> Result<String, Error> {
        let file = self.file_for_read(key.as_str())?;
        let mut buf_reader = BufReader::new(file);

        let mut value = String::new();
        buf_reader.read_to_string(&mut value)?;

        Ok(value)
    }

    fn set(&self, key: String, value: String) -> Result<(), Error> {
        let file = self.file_for_write(key.as_str())?;
        let mut buf_writer = BufWriter::new(file);

        buf_writer.write(value.as_bytes())?;

        Ok(())
    }

    fn delete(&self, key: String) -> Result<(), Error> {
        let file_path = self.path_to_file(key.as_str());
        fs::remove_file(file_path)?;

        Ok(())
    }
}
