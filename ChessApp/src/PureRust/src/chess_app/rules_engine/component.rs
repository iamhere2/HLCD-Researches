use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex}};

use crate::chess_app::data::{Turn, RuleViolation, BoardState};

use super::interface::{RulesEngineInterface, RulesEngineProvider, RulesEngineAsyncProvider};

pub struct RulesEngine {

}

impl RulesEngine {
    pub fn new() -> RulesEngine {
        RulesEngine {  }
    }
}

impl RulesEngineInterface for RulesEngine {
    fn check(&self, bs: BoardState, turn: Turn) -> Result<(), RuleViolation> {
        Ok(())
    }

    fn apply(&self, bs: BoardState, turn: Turn) -> Result<BoardState, RuleViolation> {
        dbg!("Applying turn...");
        Ok(bs)
    }
}

impl RulesEngineProvider for RulesEngine {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn RulesEngineInterface>> {
        it
    }
}

impl RulesEngineAsyncProvider for RulesEngine {
    fn get(it: Arc<Mutex<Self>>) -> Arc<Mutex<dyn RulesEngineInterface + Send + Sync>> {
        it
    }
}