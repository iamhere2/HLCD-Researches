use crate::hlcd_infra::file_io_interface::*;
use super::storage_interface::*;

// Stateless component
// Provides: StorageInterface
// Consumes: FileIOInterface
pub(super) struct FileStorage<'a> {
    
    // Owned dependencies
    file_io: Option<&'a Box<&'a dyn FileIOInterface>>,
}

impl<'a> FileStorage<'a> {

    pub(super) fn new() -> FileStorage<'a> {
        FileStorage { file_io: None }
    }

    // Owned dependencies
    pub(super) fn set_file_io(&mut self, file_io: &'a Box<&'a dyn FileIOInterface>) {
        self.file_io = Some(file_io);
    }

    fn get_file_io(&self) -> &Box<&dyn FileIOInterface> {
        self.file_io.as_ref().unwrap()
    }

    // Provided own interfaces
    pub(super) fn get_storage<'b>(&'b self) -> Box<&'b dyn StorageInterface>
        where 'b: 'a {
        Box::new(self as &'b dyn StorageInterface)
    }
}

impl<'a> StorageInterface for FileStorage<'a> {

    fn list_saved_games(&self) -> Result<Vec<String>, std::io::Error> {
        _ = self.get_file_io().list_files("*.game");
        todo!()
    }

    fn save_game(&self, gh: super::value_types::GameHistory, name: &str) -> Result<(), std::io::Error> {
        todo!()
    }

    fn load_game(&self, name: &str) -> Result<super::value_types::GameHistory, std::io::Error> {
        todo!()
    }
}