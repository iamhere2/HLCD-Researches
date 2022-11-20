use std::{rc::Rc, cell::RefCell, error::Error, fmt::Display};

use strum::Display;

use crate::chess_app::data::{Turn, RuleViolation};

#[derive(Clone, Debug)]
pub enum TurnError {
    RuleViolation(RuleViolation),
    Other(String)
}

impl Error for TurnError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for TurnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TurnError::RuleViolation(rv) => rv.rule_violation.to_string(),
            TurnError::Other(s) => s.to_string(),
        };
        write!(f, "{s}")
    }
}


pub trait TurnCmdHandlerInterface {
    fn make_turn(&self, turn: Turn) -> Result<(), TurnError>;
}

pub trait TurnCmdHandlerProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn TurnCmdHandlerInterface>>; 
}
