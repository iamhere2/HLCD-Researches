use std::{fmt::Display, error::Error};
use crate::chess_app::console_ui::data::command::Command;

#[derive(Clone, Debug)]
pub struct CmdError(pub String);

impl Error for CmdError {
}

impl Display for CmdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
} 

hlcd::define! {
    interface GameCmdHandler {
        fn execute(&self, cmd: Command) -> Result<(), CmdError>;
    }
}
