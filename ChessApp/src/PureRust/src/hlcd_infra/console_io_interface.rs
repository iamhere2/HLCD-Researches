use std::io::*;

pub trait ConsoleIOInterface {
    fn get_stdin(&self) -> Stdin;
    fn get_stdout(&self) -> Stdout;
    fn get_stderr(&self) -> Stderr;
}