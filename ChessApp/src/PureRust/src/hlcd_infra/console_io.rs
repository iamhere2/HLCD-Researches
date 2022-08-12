use std::io::*;
use super::console_io_interface::*;

pub(super) struct Console {}

impl Console {
    pub fn new() -> Console {
        Console { }
    }
}

impl ConsoleIOInterface for Console {
    fn get_stdin(&self) -> Stdin {
        stdin()
    }

    fn get_stdout(&self) -> Stdout {
        stdout()
    }

    fn get_stderr(&self) -> Stderr {
        stderr()
    }
}
