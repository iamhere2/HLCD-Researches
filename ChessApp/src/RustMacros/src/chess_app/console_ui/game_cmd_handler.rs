pub mod interface {
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
}

pub mod component {
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
                        Command::NewGame(c) => { 
                            self.game_flow_mut().new_game(c); 
                        },
                        Command::ListGames => {
                            let con = self.console_io();
                            for name in self.storage().list_saved_games().unwrap() {
                                con.write(format!("{name}\n").as_str());
                            }
                        },
                        Command::LoadGame(name) => {
                            let (gh, c) = self.storage().load_game(name.as_str())?;
                            self.game_flow_mut().start_from(gh, c); 
                        },
                        Command::DeleteGame(name) => {
                            self.storage().delete_game(&name)?
                        },
                        Command::SaveGame(name) => {
                            let flow = self.game_flow();
                            if let Some(gh) = flow.game_history() {
                                self.storage().save_game(gh, flow.player_a_color().unwrap(), name.as_str())?;
                                let con = self.console_io();
                                con.write(format!("Game saved to '{name}'...\n").as_str());
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
}