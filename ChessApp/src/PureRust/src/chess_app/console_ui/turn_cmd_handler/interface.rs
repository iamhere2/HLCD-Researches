use std::{rc::Rc, cell::RefCell};

use crate::chess_app::data::Turn;

#[derive(Clone, Debug )]
pub struct TurnError(String);

pub trait TurnCmdHandlerInterface {
    fn make_turn(&self, turn: Turn) -> Result<(), TurnError>;
}

pub trait TurnCmdHandlerProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn TurnCmdHandlerInterface>>; 
}
