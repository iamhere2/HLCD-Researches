use std::{sync::{Arc, Mutex}, rc::Rc, cell::RefCell};

use lazy_static::__Deref;

use crate::chess_app::{interactive_player_adapter::interface::InteractivePlayerAdapterInterface, data::Turn};

use super::interface::*;

pub struct TurnCmdHandler {
    interactive_player_adapter: Arc<Mutex<dyn InteractivePlayerAdapterInterface + Send + Sync>>
} 

impl TurnCmdHandler {
    pub fn new(interactive_player_adapter: &Arc<Mutex<dyn InteractivePlayerAdapterInterface + Send + Sync>>) -> TurnCmdHandler {
        let interactive_player_adapter = Arc::clone(interactive_player_adapter);
        TurnCmdHandler { interactive_player_adapter }
    }
}

impl TurnCmdHandlerProvider for TurnCmdHandler {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn TurnCmdHandlerInterface>> {
        it
    }
}

impl TurnCmdHandlerInterface for TurnCmdHandler {
    fn make_turn(&self, turn: Turn) -> Result<(), TurnError> {
        dbg!("Making turn...");
        let adapter = self.interactive_player_adapter.lock();
        let adapter = adapter.unwrap();
        let adapter = adapter.deref();
        adapter.make_turn(dbg!(turn));
        Ok(())
    }
}