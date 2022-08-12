// Republished children modules with interfaces
pub mod console_io_interface;
pub mod file_io_interface;
pub mod console_app_interface;

// Children modules with implementations
mod file_io;
mod console_io;

use lazy_static::lazy_static;
use file_io::*;
use console_io::*;
use console_io_interface::*;
use file_io_interface::*;

lazy_static! {
    static ref FILE_IO: Box<FileIO> = {
        Box::new(FileIO::new())
    };  

    static ref CONSOLE_IO: Box<Console> = {
        Box::new(Console::new())
    };  
}

pub(super) fn get_file_io() -> Box<&'static dyn FileIOInterface> {
    Box::new(FILE_IO.as_ref() as &dyn FileIOInterface)
}

pub(super) fn get_console_io() -> Box<&'static dyn ConsoleIOInterface> {
    Box::new(CONSOLE_IO.as_ref() as &dyn ConsoleIOInterface)
}