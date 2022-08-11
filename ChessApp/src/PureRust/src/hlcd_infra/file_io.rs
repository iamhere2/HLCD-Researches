use ::std::io::Error;

pub trait FileIOInterface {
    fn write_file(&self, name: &str, content: &str) -> Result<(), Error>;
    fn list_files(&self, ) -> Result<Vec<String>, Error>;
    fn read_file(&self, name: &str) -> Result<String, Error>;
}

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

    fn list_files(&self, ) -> Result<Vec<String>, Error> {
        todo!()
    }

    fn read_file(&self, name: &str) -> Result<String, Error> {
        todo!()
    }
}