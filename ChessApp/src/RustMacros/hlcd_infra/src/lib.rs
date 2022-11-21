// Republished children modules with interfaces
pub mod console_io_interface;
pub mod file_io_interface;
pub mod console_app_interface;

// Children modules with implementations
mod file_io;
mod console_io;

use std::cell::RefCell;
use std::rc::Rc;
use file_io::*;
use console_io::*;
use console_io_interface::*;
use file_io_interface::*;

pub fn get_file_io() -> FileIORef {
    Rc::new(RefCell::new(FileIO::new()))
}

pub fn get_console_io() -> ConsoleIORef {
    Rc::new(RefCell::new(Console::new()))
}