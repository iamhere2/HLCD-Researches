mod value_types;
mod storage_interface;
mod file_storage;
mod console_ui;

use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

use crate::hlcd_infra::console_app_interface::*;
use crate::hlcd_infra::console_io_interface::*;
use crate::hlcd_infra::file_io_interface::*;

use console_ui::ConsoleUI;
use file_storage::FileStorage;

use self::storage_interface::StorageProvider;

// Root component
// Provides: ConsoleApp (delegated)
// Consumes: ConsoleIO, FileIO
// Children: ConsoleUI, FileStorage
pub struct ChessApp {
    // Owned state
    // -

    // Owned dependencies
    // -

    // Children components
    console_ui: Rc<RefCell<ConsoleUI>>,
    file_storage: Rc<RefCell<FileStorage>>
}

impl ChessApp {

    // Constructor with dependencies
    pub(super) fn new(
        console_io: Rc<RefCell<dyn ConsoleIOInterface>>,
        file_io: Rc<RefCell<dyn FileIOInterface>>
    ) -> ChessApp {
        // Create children components
        let file_storage = Rc::new(RefCell::new(FileStorage::new(Rc::clone(&file_io))));
        let storage_interface = StorageProvider::get(Rc::clone(&file_storage));
        let console_ui = Rc::new(RefCell::new(ConsoleUI::new(Rc::clone(&console_io), Rc::clone(&storage_interface))));

        // Instantiate this component and assign children
        let chess_app = ChessApp { console_ui, file_storage };
        
        chess_app
    }

    // Component accessors for internal usage
    fn console_ui(&self) -> Ref<ConsoleUI> {
        self.console_ui.borrow()
    }

    fn console_ui_mut(&self) -> RefMut<ConsoleUI> {
        self.console_ui.borrow_mut()
    }
    
    // Owned dependencies
    // -
}

// Owned provided interfaces
// -

// Provided interfaces, delegated to children
impl ConsoleAppProvider for ChessApp {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn ConsoleAppInterface>> {
        ConsoleAppProvider::get(Rc::clone(&it.borrow().console_ui))
    }
}
