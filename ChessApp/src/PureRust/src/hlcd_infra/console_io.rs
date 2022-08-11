use ::std::io::*;

pub trait ConsoleIOInterface {
    fn get_stdin(&self) -> Stdin;
    fn get_stdout(&self) -> Stdout;
    fn get_stderr(&self) -> Stderr;
}

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
