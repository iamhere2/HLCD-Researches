pub mod console_io;
pub mod file_io;
pub mod console_app;

use lazy_static::lazy_static;

use self::file_io::*;
use self::console_io::*;

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