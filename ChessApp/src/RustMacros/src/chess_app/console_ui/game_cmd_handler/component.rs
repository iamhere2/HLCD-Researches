use std::{rc::Rc, cell::RefCell};

use crate::{hlcd_infra::console_io_interface::*, chess_app::{storage_interface::*, game_flow::interface::*, console_ui::data::command::Command}};

use super::interface::*;

// Component
// Consume: ConsoleIO, GameFlow, Storage
pub struct GameCmdHandler {
    // Dependencies
    console_io: Rc<RefCell<dyn ConsoleIOInterface>>,
    game_flow: Rc<RefCell<dyn GameFlowInterface>>,
    storage: Rc<RefCell<dyn StorageInterface>>
} 

impl GameCmdHandler {
    pub fn new(
        console_io: &Rc<RefCell<dyn ConsoleIOInterface>>,
        game_flow: &Rc<RefCell<dyn GameFlowInterface>>,
        storage: &Rc<RefCell<dyn StorageInterface>>,
    ) -> GameCmdHandler {
        GameCmdHandler {  
            console_io: Rc::clone(console_io),
            game_flow: Rc::clone(game_flow),
            storage: Rc::clone(storage)
        }
    }
}

impl GameCmdHandlerProvider for GameCmdHandler {
    fn get(it: Rc<RefCell<Self>>) -> Rc<RefCell<dyn GameCmdHandlerInterface>> {
        it
    }
}

impl GameCmdHandlerInterface for GameCmdHandler {
    fn execute(&self, cmd: Command) -> Result<(), CmdError> {
        match cmd {
            Command::MakeTurn(_) => unreachable!(),
            Command::Exit => unreachable!(),
            Command::Help => unreachable!(),
            Command::NewGame(c) => {
                dbg!("New game");
                dbg!(c);
                RefCell::borrow_mut(&self.game_flow).new_game(c);
            },
            Command::ListGames => todo!(),
            Command::LoadGame(_) => todo!(),
            Command::DeleteGame(_) => todo!(),
            Command::SaveGame(_) => todo!(),
        }
        Ok(())
    }
}