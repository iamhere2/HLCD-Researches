use std::io::Error;

pub trait FileIOInterface {
    fn write_file(&self, name: &str, content: &str) -> Result<(), Error>;
    fn list_files(&self, pattern: &str) -> Result<Vec<String>, Error>;
    fn read_file(&self, name: &str) -> Result<String, Error>;
}
