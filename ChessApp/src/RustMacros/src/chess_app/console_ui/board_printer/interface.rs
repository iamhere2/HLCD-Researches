use std::{io::Error, rc::Rc, cell::RefCell};
use crate::chess_app::data::board::BoardState;

use super::super::data::*;

pub trait BoardPrinterInterface {
    fn print(&self, board: &BoardState);
}

pub trait BoardPrinterProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn BoardPrinterInterface>>; 
}
