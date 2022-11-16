use std::{rc::Rc, cell::RefCell};

use lazy_static::__Deref;

use crate::chess_app::{data::Turn, rules_engine::interface::RulesEngineInterface, game_flow::interface::FlowPlayInterface};

use super::interface::*;

pub struct TurnCmdHandler {
    rules_engine_interface: Rc<RefCell<dyn RulesEngineInterface>>,
    flow_play_interface: Rc<RefCell<dyn FlowPlayInterface>>
} 

impl TurnCmdHandler {
    pub fn new(
        rules_engine_interface: &Rc<RefCell<dyn RulesEngineInterface>>,
        flow_play_interface: &Rc<RefCell<dyn FlowPlayInterface>>
    ) -> TurnCmdHandler {
        let rules_engine_interface = Rc::clone(&rules_engine_interface);
        let flow_play_interface = Rc::clone(&flow_play_interface);
        TurnCmdHandler { rules_engine_interface, flow_play_interface }
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
        self.flow_play_interface.borrow().make_turn(turn);
        Ok(())
    }
}