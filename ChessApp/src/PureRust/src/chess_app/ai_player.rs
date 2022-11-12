use std::{cell::RefCell, rc::Rc, sync::{Mutex, mpsc::{Sender, Receiver}, Arc}};

use super::{player_interface::{AsyncPlayerProvider, AsyncPlayerInterface}, data::{BoardState, Turn, RuleViolation}};

pub(super) struct AiPlayer {

}

impl AiPlayer {
    pub(super) fn new() -> AiPlayer {
        AiPlayer {  }
    }
}

impl AsyncPlayerProvider for AiPlayer {
    fn get(it: Arc<Mutex<Self>>) -> Arc<Mutex<dyn AsyncPlayerInterface + Send + Sync>> {
        it
    }
}

impl AsyncPlayerInterface for AiPlayer {
    fn next_turn_request_sender(&self) -> Arc<Mutex<Sender<BoardState>>> {
        todo!()
    }

    fn next_turn_receiver(&self) -> Arc<Mutex<Receiver<Turn>>> {
        todo!()
    }

    fn rule_violation_sender(&self) -> Arc<Mutex<Sender<RuleViolation>>> {
        todo!()
    }
}