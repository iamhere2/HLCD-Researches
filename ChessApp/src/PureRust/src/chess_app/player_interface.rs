use std::{rc::Rc, cell::RefCell};
use std::sync::mpsc::Receiver;

use super::data::{BoardState, Turn};

pub trait PlayerInterface {
    fn receiver(&self) -> Receiver<Turn>;
    fn next_turn(&self, bs: &BoardState);
}

pub(super) trait PlayerProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn PlayerInterface>>; 
}
