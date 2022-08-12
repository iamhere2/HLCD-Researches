use std::io::Write;

// For provided interfaces
use crate::hlcd_infra::console_app_interface::*;

// For consumed interfaces
use crate::hlcd_infra::console_io_interface::*;
use super::storage_interface::*;

// Component
// Provides: ConsoleApp
// Consumes: ConsoleUI, Storage
pub(super) struct ConsoleUI<'a> {

    // Owned dependencies
    console_io: Option<&'a Box<&'a dyn ConsoleIOInterface>>,
    storage: Option<&'a Box<&'a dyn StorageInterface>>
}

impl<'a> ConsoleUI<'a> {
    pub(super) fn new() -> ConsoleUI<'a> {
        ConsoleUI { console_io: None, storage: None }
    }

    // Owned dependencies
    pub(super) fn set_console_io(&mut self, console_io: &'a Box<&'a dyn ConsoleIOInterface>) {
        self.console_io = Some(console_io);
    }

    fn get_console_io(&self) -> &Box<&dyn ConsoleIOInterface> {
        self.console_io.as_ref().unwrap()
    }

    pub(super) fn set_storage(&mut self, storage: &'a Box<&'a dyn StorageInterface>) {
        self.storage = Some(storage);
    }

    fn get_storage(&self) -> &Box<&dyn StorageInterface> {
        self.storage.as_ref().unwrap()
    }


    // Provided interfaces
    pub(super) fn get_console_app(&self) -> Box<&dyn ConsoleAppInterface> {
        Box::new(self as &dyn ConsoleAppInterface)
    }
}

impl<'a> ConsoleAppInterface for ConsoleUI<'a> {
    fn run(&self) -> i32 {
        _ = self.get_console_io().get_stdout().write_fmt(format_args!("Hello, World!"));
        0
    }
}
