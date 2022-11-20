#[cfg(test)]
mod tests;

use std::io::Error;
use super::file_io_interface::*;

pub(super) struct FileIO {}

impl FileIO {
    pub(super) fn new() -> FileIO {
        FileIO { }
    }
}

impl FileIOInterface for FileIO {
    fn write_file(&self, path: &str, content: &str) -> Result<(), Error> {
        Ok(std::fs::write(path, content)?)
    }

    fn list_files(&self, directory: &str, pattern: &str) -> Result<Vec<String>, Error> {
        let read_dir = std::fs::read_dir(directory)?;
        let file_names = read_dir.map(|e| e.unwrap().file_name().to_str().unwrap().to_string());
        let wc = wildmatch::WildMatch::new(pattern);
        Ok(file_names.filter(|s| wc.matches(s)).collect())
    }

    fn read_file(&self, path: &str) -> Result<String, Error> {
        Ok(std::fs::read_to_string(path)?)
    }

    fn delete_file(&self, path: &str) -> Result<(), Error> {
        Ok(std::fs::remove_file(path)?)
    }

    fn get_current_directory(&self) -> Result<String, Error> {
        std::env::current_dir().map(|path| path.as_os_str().to_str().unwrap().to_string())
    }
}