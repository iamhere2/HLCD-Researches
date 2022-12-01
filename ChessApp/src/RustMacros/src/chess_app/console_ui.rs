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

        provides { ConsoleApp via command_cycle }

        requires {
            ConsoleIO,
            Storage,
            RulesEngine,
            GameFlow,
            FlowPlay 
        }

        children {
            command_parser: CommandParser(),
            board_printer: BoardPrinter(ConsoleIO),
            game_cmd_handler: GameCmdHandler(ConsoleIO, GameFlow, Storage),
            turn_cmd_handler: TurnCmdHandler(RulesEngine, FlowPlay),
            command_cycle: CommandCycle(
                ConsoleIO,
                command_parser for CommandParser,
                turn_cmd_handler for TurnCmdHandler,
                game_cmd_handler for GameCmdHandler,
                board_printer for BoardPrinter,
                GameFlow
            )
        }
    }
}
