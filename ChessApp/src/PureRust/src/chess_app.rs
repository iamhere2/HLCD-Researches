mod value_types;
mod storage_interface;
mod file_storage;
mod console_ui;

use crate::hlcd_infra::console_app_interface::*;
use crate::hlcd_infra::console_io_interface::*;
use crate::hlcd_infra::file_io_interface::*;

use console_ui::ConsoleUI;
use file_storage::FileStorage;

use self::storage_interface::StorageInterface;

// Stateless root component
// Provides: ConsoleApp
// Consumes: ConsoleUI, FileIO
pub struct ChessApp<'a> {
    // Owned state
    // -

    // Owned dependencies
    // -

    // Children components
    console_ui: Box<ConsoleUI<'a>>,
    file_storage: Box<FileStorage<'a>>,

    // Interfaces, provided by children
    // file_storage_as_storage: Box<&'a dyn StorageInterface>
}

impl<'a> ChessApp<'a> {

    pub(super) fn new() -> ChessApp<'a> {
        // Create children components:
        //   - mutable to set dependencies
        //   - boxed, to save in fields 
        let mut console_ui = Box::new(ConsoleUI::new());
        let mut file_storage = Box::new(FileStorage::new());

        // Interfaces, provided by children
        // let file_storage_as_storage = file_storage.get_storage();

        // Instantiate this component and assign children
        let chess_app = ChessApp { console_ui, file_storage /*, file_storage_as_storage */ };
        
        // Link children components to each other
        ////////// ?????? console_ui.set_storage(&chess_app.file_storage_as_storage);
        // -
        
        chess_app
    }

    // Dependencies, delegated to children
    pub(super) fn set_console_io(&mut self, console_io: &'a Box<&'a dyn ConsoleIOInterface>) {
        self.console_ui.set_console_io(console_io);
    }

    pub(super) fn set_file_io(&mut self, file_io: &'a Box<&'a dyn FileIOInterface>) {
        self.file_storage.set_file_io(file_io);
    }

    // Provided interfaces, delegated to children
    pub(super) fn get_console_app(&self) -> Box<&dyn ConsoleAppInterface> {
        self.console_ui.get_console_app()
    }

    // Owned provided interfaces
    // -

    // Owned dependencies
    // -
}
