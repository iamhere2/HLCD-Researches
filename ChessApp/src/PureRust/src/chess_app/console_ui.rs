use std::io::Write;

use crate::hlcd_infra::console_app::ConsoleAppInterface;
use crate::hlcd_infra::console_io::ConsoleIOInterface;

// Component
pub(super) struct ConsoleUI<'a> {

    // Owned dependencies
    console_io: Option<&'a Box<&'a dyn ConsoleIOInterface>>
}

impl<'a> ConsoleUI<'a> {
    pub(super) fn new() -> ConsoleUI<'a> {
        ConsoleUI { console_io: None }
    }

    // Owned dependencies
    pub(super) fn set_console_io(&mut self, console_io: &'a Box<&'a dyn ConsoleIOInterface>) {
        self.console_io = Some(console_io);
    }

    fn get_console_io(&self) -> &Box<&dyn ConsoleIOInterface> {
        self.console_io.as_ref().unwrap()
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
