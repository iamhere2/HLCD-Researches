use std::{rc::Rc, cell::RefCell};

use crate::chess_app::data::{BoardState, Turn, RuleViolation, Color};

use hlcd_macros::*;

interface!{ RulesEngine {
    fn apply(&self, bs: &BoardState, player: Color, turn: Turn) -> Result<BoardState, RuleViolation>;
}}

pub trait RulesEngineProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn RulesEngineInterface>>; 
}
