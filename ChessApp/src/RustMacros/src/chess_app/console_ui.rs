// Children modules
mod data;
mod board_printer;
mod command_cycle;
mod command_parser;
mod game_cmd_handler;
mod turn_cmd_handler;

// Provided interfaces
use crate::hlcd_infra::console_app_interface::*;

// Consumed interfaces
use crate::hlcd_infra::console_io_interface::*;
use super::storage_interface::*;
use super::game_flow::interface::*;
use super::rules_engine::interface::*;

// Children components and interfaces
use self::command_cycle::component::*;
use self::board_printer::{component::*, interface::*};
use self::command_parser::{component::*, interface::*};
use self::game_cmd_handler::{component::*, interface::*};
use self::turn_cmd_handler::{component::*, interface::*};

hlcd::define! {
    component ConsoleUI {

        provides { ConsoleApp by command_cycle }

        requires {
            ConsoleIO,
            Storage,
            RulesEngine,
            GameFlowControl,
            ForActivePlayer 
        }

        children {
            command_parser: CommandParser(),
            board_printer: BoardPrinter(ConsoleIO),
            game_cmd_handler: GameCmdHandler(ConsoleIO, GameFlowControl, Storage),
            turn_cmd_handler: TurnCmdHandler(RulesEngine, ForActivePlayer),
            command_cycle: CommandCycle(
                ConsoleIO,
                CommandParser by command_parser,
                TurnCmdHandler by turn_cmd_handler,
                GameCmdHandler by game_cmd_handler,
                BoardPrinter by board_printer,
                GameFlowControl
            )
        }
    }
}
