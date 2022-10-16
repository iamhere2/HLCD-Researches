use std::{io::Error, rc::Rc, cell::RefCell};
use super::super::data::*;

pub(super) trait BoardPrinterInterface {
    fn print(&self, board: BoardState);
}

pub(super) trait BoardPrinterProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn BoardPrinterInterface>>; 
}
