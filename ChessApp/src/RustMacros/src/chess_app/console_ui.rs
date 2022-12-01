// Children modules
mod data;
mod board_printer;
mod command_cycle;
mod command_parser;
mod game_cmd_handler;
mod turn_cmd_handler;

use std::borrow::BorrowMut;
use std::rc::Rc;
use std::cell::{ Ref, RefCell, RefMut };
use std::io::Write;
use enum_iterator::IntoEnumIterator;

// Provided interfaces
use crate::hlcd_infra::console_app_interface::*;

// Data structures
use super::data::board;

// Consumed interfaces
use crate::hlcd_infra::console_io_interface::*;
use super::storage_interface::{*, self};
use super::game_flow::interface::*;
use super::rules_engine::interface::*;

// Children components and interfaces
use self::command_cycle::component::*;
use self::board_printer::{component::*, interface::*};
use self::command_parser::{component::*, interface::*};
use self::game_cmd_handler::{component::*, interface::*};
use self::turn_cmd_handler::{component::*, interface::*};


// hlcd::define! {
//     component Test {
//         requires {
//             ConsoleIO
//         }
//         children {
//             board_printer: BoardPrinter(ConsoleIO)
//         }

//     }
// }

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


// pub(super) type ConsoleUIInstanceRef = Rc<RefCell<ConsoleUI>>; 

// // Component
// // Provides: ConsoleApp
// // Consumes: ConsoleUI, Storage
// pub(super) struct ConsoleUI {

//     // Dependencies
//     console_io: ConsoleIORef,
//     storage: StorageRef,
//     rules_engine: RulesEngineRef,
//     game_flow: GameFlowRef,
//     flow_play: FlowPlayRef,

//     // Children components 
//     board_printer: Rc<RefCell<BoardPrinter>>,
//     command_cycle: Rc<RefCell<CommandCycle>>,
//     command_parser: Rc<RefCell<CommandParser>>,
//     turn_cmd_handler: Rc<RefCell<TurnCmdHandler>>,
//     game_cmd_handler: Rc<RefCell<GameCmdHandler>>,

//     // Children components' interfaces
//     board_printer_interface: BoardPrinterRef,
//     command_parser_interface: CommandParserRef,
//     command_cycle_console_app_interface: ConsoleAppRef,
//     turn_cmd_handler_interface: TurnCmdHandlerRef,
//     game_cmd_handler_interface: GameCmdHandlerRef,
// }

// impl ConsoleUI {
//     // Constructor with dependencies
//     pub(super) fn new(
//         console_io: &ConsoleIORef,
//         storage: &StorageRef,
//         game_flow: &GameFlowRef,
//         flow_play: &FlowPlayRef,
//         rules_engine: &RulesEngineRef
//     ) 
//     -> ConsoleUI {
//         let console_io = Rc::clone(&console_io); 
//         let storage = Rc::clone(&storage);
//         let rules_engine = Rc::clone(&rules_engine);
//         let game_flow = Rc::clone(&game_flow);
//         let flow_play = Rc::clone(&flow_play);

//         let board_printer = Rc::new(RefCell::new(BoardPrinter::new(
//             &console_io
//         )));
//         let board_printer_interface = BoardPrinterProvider::get(Rc::clone(&board_printer));

//         let command_parser = Rc::new(RefCell::new(CommandParser::new()));
//         let command_parser_interface = CommandParserProvider::get(Rc::clone(&command_parser));

//         let turn_cmd_handler = Rc::new(RefCell::new(TurnCmdHandler::new(&rules_engine, &flow_play))); 
//         let turn_cmd_handler_interface = TurnCmdHandlerProvider::get(Rc::clone(&turn_cmd_handler));

//         let game_cmd_handler = Rc::new(RefCell::new(GameCmdHandler::new(&console_io, &game_flow, &storage))); 
//         let game_cmd_handler_interface = GameCmdHandlerProvider::get(Rc::clone(&game_cmd_handler));

//         let command_cycle = Rc::new(RefCell::new(CommandCycle::new(
//             &console_io,
//             &command_parser_interface,
//             &turn_cmd_handler_interface,
//             &game_cmd_handler_interface,
//             &board_printer_interface,
//             &game_flow
//         ))); 

//         let command_cycle_console_app_interface = ConsoleAppProvider::get(Rc::clone(&command_cycle));

//         ConsoleUI { 
//             console_io, 
//             storage, 
//             board_printer, 
//             command_cycle, 
//             command_parser,
//             turn_cmd_handler,
//             game_cmd_handler,
//             board_printer_interface,
//             command_parser_interface,
//             game_cmd_handler_interface,
//             turn_cmd_handler_interface,
//             command_cycle_console_app_interface,
//             game_flow,
//             flow_play,
//             rules_engine
//         }
//     }
// }

// impl ConsoleAppProvider for ConsoleUI {
//     fn get(it: Rc<RefCell<ConsoleUI>>) -> ConsoleAppRef { 
//         ConsoleAppProvider::get(Rc::clone(&RefCell::borrow(&it).command_cycle)) 
//     }
// }
