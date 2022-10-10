// Children modules
mod board_printer;
mod command_cycle;
mod command_parser;
mod game_cmd_handler;
mod turn_cmd_handler;

use std::rc::Rc;
use std::cell::{ Ref, RefCell, RefMut };
use std::io::Write;

// For provided interfaces
use crate::hlcd_infra::console_app_interface::*;

// For consumed interfaces
use crate::hlcd_infra::console_io_interface::*;
use super::storage_interface::*;

// Component
// Provides: ConsoleApp
// Consumes: ConsoleUI, Storage
pub(super) struct ConsoleUI {

    // Owned dependencies
    console_io: Rc<RefCell<dyn ConsoleIOInterface>>,
    storage: Rc<RefCell<dyn StorageInterface>>
}

impl ConsoleUI {
    // Constructor with dependencies
    pub(super) fn new(
        console_io: Rc<RefCell<dyn ConsoleIOInterface>>,
        storage: Rc<RefCell<dyn StorageInterface>>) 
    -> ConsoleUI {
        ConsoleUI { console_io: console_io.clone(), storage: storage.clone() }
    }

    // Owned dependencies access, for internal use
    fn console_io(&self) -> Ref<dyn ConsoleIOInterface> {
        self.console_io.borrow()
    }

    fn console_io_mut(&self) -> RefMut<dyn ConsoleIOInterface> {
        self.console_io.borrow_mut()
    }

    fn storage(&self) -> Ref<dyn StorageInterface> {
        self.storage.borrow()
    }

    fn storage_mut(&self) -> RefMut<dyn StorageInterface> {
        self.storage.borrow_mut()
    }
}

// Provided interface - implemented by itself
impl ConsoleAppProvider for ConsoleUI {
    fn get(it: Rc<RefCell<ConsoleUI>>) -> Rc<RefCell<dyn ConsoleAppInterface>> { it }
}

impl ConsoleAppInterface for ConsoleUI {
    fn run(&self) -> i32 {
        _ = self.console_io().get_stdout().write_fmt(format_args!("Hello, World!"));
        0
    }
}
