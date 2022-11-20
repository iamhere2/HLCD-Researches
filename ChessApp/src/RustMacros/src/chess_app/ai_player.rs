use std::{cell::RefCell, rc::Rc};

use super::{player_interface::*, data::{BoardState, Turn, RuleViolation}};

pub(super) struct AiPlayer {
}

impl AiPlayer {
    pub(super) fn new() -> AiPlayer {
        AiPlayer { 
        }
    }
}

impl SyncPlayerProvider for AiPlayer {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn SyncPlayerInterface>> {
        it
    }
}

impl SyncPlayerInterface for AiPlayer {
    fn turn_request(&self, bs: &BoardState) -> Turn {
        todo!()
    }
}