use std::{cell::{RefCell, Ref, RefMut}, rc::Rc};

use crate::hlcd_infra::file_io_interface::*;
use super::storage_interface::*;

// Stateless component
// Provides: StorageInterface
// Consumes: FileIOInterface
pub(super) struct FileStorage {
    
    // Owned dependencies
    file_io: Rc<RefCell<dyn FileIOInterface>>,
}

impl FileStorage {

    // Constructor with dependencies
    pub(super) fn new(
        file_io: Rc<RefCell<dyn FileIOInterface>>
        ) -> FileStorage {
        FileStorage { file_io: file_io.clone() }
    }

    // Owned dependencies access for internal usage
    fn file_io(&self) -> Ref<dyn FileIOInterface> {
        self.file_io.borrow()
    }

    fn file_io_mut(&self) -> RefMut<dyn FileIOInterface> {
        self.file_io.borrow_mut()
    }
}

// Provided own interfaces
impl StorageProvider for FileStorage {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn StorageInterface>> { it }
}

impl StorageInterface for FileStorage {

    fn list_saved_games(&self) -> Result<Vec<String>, std::io::Error> {
        _ = self.file_io().list_files("*.game");
        todo!()
    }

    fn save_game(&self, gh: super::data::concepts::GameHistory, name: &str) -> Result<(), std::io::Error> {
        todo!()
    }

    fn load_game(&self, name: &str) -> Result<super::data::concepts::GameHistory, std::io::Error> {
        todo!()
    }
}