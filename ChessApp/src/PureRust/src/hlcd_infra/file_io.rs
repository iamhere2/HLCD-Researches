use std::io::Error;
use super::file_io_interface::*;

pub(super) struct FileIO {}

impl FileIO {
    pub(super) fn new() -> FileIO {
        FileIO { }
    }
}

impl FileIOInterface for FileIO {
    fn write_file(&self, name: &str, content: &str) -> Result<(), Error> {
        todo!()
    }

    fn list_files(&self, pattern: &str) -> Result<Vec<String>, Error> {
        todo!()
    }

    fn read_file(&self, name: &str) -> Result<String, Error> {
        todo!()
    }
}