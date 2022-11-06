use super::interface::*;

pub struct TurnCmdHandler {

} 

impl TurnCmdHandler {
    pub fn new() -> TurnCmdHandler {
        TurnCmdHandler {  }
    }
}

impl TurnCmdHandlerProvider for TurnCmdHandler {
    fn get(it: std::rc::Rc<std::cell::RefCell<Self>>) -> std::rc::Rc<std::cell::RefCell<dyn TurnCmdHandlerInterface>> {
        it
    }
}

impl TurnCmdHandlerInterface for TurnCmdHandler {
    fn make_turn(&self, turn: crate::chess_app::data::Turn) -> Result<(), TurnError> {
        println!("TurnCmdHandler: Not implemented!");
        Ok(())
    }
}