use std::io::Error;

pub trait FileIOInterface {
    fn write_file(&self, name: &str, content: &str) -> Result<(), Error>;
    fn list_files(&self, directory: &str, pattern: &str) -> Result<Vec<String>, Error>;
    fn read_file(&self, name: &str) -> Result<String, Error>;
    fn delete_file(&self, name: &str) -> Result<(), Error>;
    fn get_current_directory(&self) -> Result<String, Error>;
}
