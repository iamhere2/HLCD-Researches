// Children modules
mod data;
mod storage_interface;
mod player_interface;
mod rules_engine;
mod file_storage;
mod console_ui;
mod game_flow;
mod ai_player;

// Required interfaces
use crate::hlcd_infra::console_app_interface::*;
use crate::hlcd_infra::console_io_interface::*;
use crate::hlcd_infra::file_io_interface::*;

// Children components and interfaces
use self::ai_player::*;
use self::player_interface::*;
use self::console_ui::*;
use self::file_storage::*;
use self::game_flow::{component::*, interface::*};
use self::rules_engine::{component::*, interface::*};
use self::storage_interface::*;

hlcd::define! {
    component ChessApp {
        provides { ConsoleApp by console_ui }
        requires { ConsoleIO, FileIO }
        children {
            file_storage: FileStorage(FileIO),
            ai_player: AiPlayer(),
            rules_engine: RulesEngine(),
            game_flow: ActivePassiveSyncGameFlow(
                PassiveSyncPlayer by ai_player,
                RulesEngine by rules_engine 
            ),
            console_ui: ConsoleUI(
                ConsoleIO,
                Storage by file_storage,
                RulesEngine by rules_engine,
                GameFlowControl by game_flow,
                ForActivePlayer by game_flow     
            )
        }
    }
}