use std::{rc::Rc, cell::RefCell};

pub trait ConsoleAppInterface {
    fn run(&self) -> i32; 
}

pub trait ConsoleAppProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn ConsoleAppInterface>>; 
}
