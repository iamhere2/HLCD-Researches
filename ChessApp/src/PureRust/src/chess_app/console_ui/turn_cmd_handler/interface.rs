use std::{rc::Rc, cell::RefCell, error::Error, fmt::Display};

use strum::Display;

use crate::chess_app::data::Turn;

#[derive(Clone, Debug)]
pub struct TurnError(String);

impl Error for TurnError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for TurnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


pub trait TurnCmdHandlerInterface {
    fn make_turn(&self, turn: Turn) -> Result<(), TurnError>;
}

pub trait TurnCmdHandlerProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn TurnCmdHandlerInterface>>; 
}
