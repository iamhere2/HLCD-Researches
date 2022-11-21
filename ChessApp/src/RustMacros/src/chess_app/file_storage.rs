mod serialization;
use serialization::{serialize, deserialize, Error as SerdeError};

use std::{cell::{RefCell, Ref, RefMut}, rc::Rc};

use crate::hlcd_infra::file_io_interface::*;
use super::{storage_interface::*, data::{GameHistory, Color}};

pub(super) type FileStorageRef = Rc<RefCell<FileStorage>>;

// Stateless component
// Provides: StorageInterface
// Consumes: FileIOInterface
pub(super) struct FileStorage {
    
    // Owned dependencies
    file_io: FileIORef,
}

impl FileStorage {

    // Constructor with dependencies
    pub(super) fn new(
        file_io: FileIORef
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
    fn get(it: Rc<RefCell<Self>>) -> StorageRef { it }
}

const EXT: &str = "chess";

fn build_file_name(name: &str) -> String {
    format!("{name}.{EXT}")
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error(format!("{}", e))
    }
}

impl From<SerdeError> for Error {
    fn from(e: SerdeError) -> Self {
        Error(format!("{}", e.0))
    }
}

impl StorageInterface for FileStorage {

    fn list_saved_games(&self) -> Result<Vec<String>, Error> {
        let io = self.file_io();
        Ok(io.list_files(&io.get_current_directory()?, format!("*.{EXT}").as_str())?)
    }

    fn save_game(&self, gh: GameHistory, color: Color, name: &str) -> Result<(), Error> {
        Ok(self.file_io().write_file(
            build_file_name(name).as_str(), 
            serialize(gh, color)?.as_str())?
        )
    }

    fn load_game(&self, name: &str) -> Result<(GameHistory, Color), Error> {
        let content = self.file_io().read_file(
            build_file_name(name).as_str())?;
        Ok(deserialize(content.as_str())?)
    }
}

