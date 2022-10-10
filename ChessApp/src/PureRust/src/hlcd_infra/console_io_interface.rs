use std::{io::*, rc::Rc, cell::RefCell};

pub trait ConsoleIOInterface {
    fn get_stdin(&self) -> Stdin;
    fn get_stdout(&self) -> Stdout;
    fn get_stderr(&self) -> Stderr;
}

pub trait ConsoleIOProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn ConsoleIOInterface>>; 
}