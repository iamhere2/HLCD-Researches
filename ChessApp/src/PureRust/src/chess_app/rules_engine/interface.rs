use std::{rc::Rc, cell::RefCell};

use crate::chess_app::data::{BoardState, Turn, RuleViolation};

pub trait RulesEngineInterface {
    fn check(&self, bs: BoardState, turn: Turn) -> Result<(), RuleViolation>;
    fn apply(&self, bs: BoardState, turn: Turn) -> Result<BoardState, RuleViolation>;
}

pub trait RulesEngineProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn RulesEngineInterface>>; 
}
