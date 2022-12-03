use crate::{hlcd_infra::console_io_interface::*, chess_app::{storage_interface::*, game_flow::interface::*, console_ui::data::command::Command}};
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
                    Command::SaveGame(_) => todo!(),
                }
                Ok(())
            }
        }

    }
}