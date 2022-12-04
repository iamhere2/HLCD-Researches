use crate::{hlcd_infra::console_io_interface::*, chess_app::{storage_interface::{*, self}, game_flow::interface::*, console_ui::data::command::Command}};
use super::interface::*;

hlcd::define! {
    component GameCmdHandler {

        requires {
            console_io: ConsoleIO,
            game_flow: GameFlowControl,
            storage: Storage
        }

        provides { GameCmdHandler }

        impl GameCmdHandler {
            fn execute(&self, cmd: Command) -> Result<(), CmdError> {
                match cmd {
                    Command::MakeTurn(_) => unreachable!(),
                    Command::Exit => unreachable!(),
                    Command::Help => unreachable!(),
                    Command::NewGame(c) => { self.game_flow_mut().new_game(c); },
                    Command::ListGames => todo!(),
                    Command::LoadGame(_) => todo!(),
                    Command::DeleteGame(_) => todo!(),
                    Command::SaveGame(name) => {
                        let flow = self.game_flow();
                        if let Some(gh) = flow.game_history() {
                            self.storage().save_game(gh, flow.player_a_color().unwrap(), name.as_str())?
                        } else {
                            return Err(CmdError("No active game to save".to_string()))
                        }
                    },
                }
                Ok(())
            }
        }

    }
}

impl From<storage_interface::Error> for CmdError {
    fn from(e: storage_interface::Error) -> Self {
        CmdError(format!("I/O error: {}", e.0))
    }
}