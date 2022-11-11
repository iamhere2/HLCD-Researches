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

// Provided interfaces
use crate::hlcd_infra::console_app_interface::*;

// Data structures
use super::data::board;
use super::game_flow::interface::GameFlowInterface;

// Consumed interfaces
use crate::hlcd_infra::console_io_interface::*;
use board_printer::component::BoardPrinter;
use board_printer::interface::BoardPrinterInterface;
use enum_iterator::IntoEnumIterator;
use super::storage_interface::*;

use self::board_printer::interface::BoardPrinterProvider;
// Children components and interfaces
use self::command_cycle::component::*;
use self::command_parser::{component::*, interface::*};
use self::game_cmd_handler::{component::*, interface::*};
use self::turn_cmd_handler::{component::*, interface::*};

// Component
// Provides: ConsoleApp
// Consumes: ConsoleUI, Storage
pub(super) struct ConsoleUI {

    // Dependencies
    console_io: Rc<RefCell<dyn ConsoleIOInterface>>,
    storage: Rc<RefCell<dyn StorageInterface>>,

    // Children components 
    board_printer: Rc<RefCell<BoardPrinter>>,
    command_cycle: Rc<RefCell<CommandCycle>>,
    command_parser: Rc<RefCell<CommandParser>>,
    turn_cmd_handler: Rc<RefCell<TurnCmdHandler>>,
    game_cmd_handler: Rc<RefCell<GameCmdHandler>>,

    // Children components' interfaces
    board_printer_interface: Rc<RefCell<dyn BoardPrinterInterface>>,
    command_parser_interface: Rc<RefCell<dyn CommandParserInterface>>,
    command_cycle_console_app_interface: Rc<RefCell<dyn ConsoleAppInterface>>,
    turn_cmd_handler_interface: Rc<RefCell<dyn TurnCmdHandlerInterface>>,
    game_cmd_handler_interface: Rc<RefCell<dyn GameCmdHandlerInterface>>,
}

impl ConsoleUI {
    // Constructor with dependencies
    pub(super) fn new(
        console_io: &Rc<RefCell<dyn ConsoleIOInterface>>,
        storage: &Rc<RefCell<dyn StorageInterface>>,
        game_flow: &Rc<RefCell<dyn GameFlowInterface>>) 
    -> ConsoleUI {
        let console_io = Rc::clone(&console_io); 
        let storage = Rc::clone(&storage);

        let board_printer = Rc::new(RefCell::new(BoardPrinter::new(
            &console_io
        )));
        let board_printer_interface = BoardPrinterProvider::get(Rc::clone(&board_printer));

        let command_parser = Rc::new(RefCell::new(CommandParser::new()));
        let command_parser_interface = CommandParserProvider::get(Rc::clone(&command_parser));

        let turn_cmd_handler = Rc::new(RefCell::new(TurnCmdHandler::new())); 
        let turn_cmd_handler_interface = TurnCmdHandlerProvider::get(Rc::clone(&turn_cmd_handler));

        let game_cmd_handler = Rc::new(RefCell::new(GameCmdHandler::new(&console_io, &game_flow, &storage))); 
        let game_cmd_handler_interface = GameCmdHandlerProvider::get(Rc::clone(&game_cmd_handler));

        let command_cycle = Rc::new(RefCell::new(CommandCycle::new(
            &console_io,
            &command_parser_interface,
            &turn_cmd_handler_interface,
            &game_cmd_handler_interface,
            &board_printer_interface
        ))); 

        let command_cycle_console_app_interface = ConsoleAppProvider::get(Rc::clone(&command_cycle));

        ConsoleUI { 
            console_io, 
            storage, 
            board_printer, 
            command_cycle, 
            command_parser,
            turn_cmd_handler,
            game_cmd_handler,
            board_printer_interface,
            command_parser_interface,
            game_cmd_handler_interface,
            turn_cmd_handler_interface,
            command_cycle_console_app_interface
        }
    }
}

// Provided interface - implemented partly by itself, partly - delegated
impl ConsoleAppProvider for ConsoleUI {
    fn get(it: Rc<RefCell<ConsoleUI>>) -> Rc<RefCell<dyn ConsoleAppInterface>> { 
        ConsoleAppProvider::get(Rc::clone(&it.borrow().command_cycle)) 
    }
}

impl ConsoleAppInterface for ConsoleUI {
    fn run(&self) -> i32 {
        self.command_cycle_console_app_interface.borrow().run()
    }
}
