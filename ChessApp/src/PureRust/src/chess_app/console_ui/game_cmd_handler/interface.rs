use std::{rc::Rc, cell::RefCell};

use crate::chess_app::console_ui::data::command::Command;

#[derive(Clone, Debug)]
pub struct CmdError(String);

pub trait GameCmdHandlerInterface {
    fn execute(&self, cmd: Command) -> Result<(), CmdError>;
}

pub trait TurnCmdHandlerProvider {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn GameCmdHandlerInterface>>; 
}
