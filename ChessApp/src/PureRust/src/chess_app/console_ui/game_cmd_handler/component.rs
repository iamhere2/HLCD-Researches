use super::interface::*;

pub struct GameCmdHandler {

} 

impl GameCmdHandler {
    pub fn new() -> GameCmdHandler {
        GameCmdHandler {  }
    }
}

impl GameCmdHandlerProvider for GameCmdHandler {
    fn get(it: std::rc::Rc<std::cell::RefCell<Self>>) -> std::rc::Rc<std::cell::RefCell<dyn GameCmdHandlerInterface>> {
        it
    }
}

impl GameCmdHandlerInterface for GameCmdHandler {
    fn execute(&self, cmd: crate::chess_app::console_ui::data::command::Command) -> Result<(), CmdError> {
        println!("GameCmdHandler: Not implemented!");
        Ok(())
    }
}