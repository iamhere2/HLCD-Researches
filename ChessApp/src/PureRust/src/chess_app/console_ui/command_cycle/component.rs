
use std::cell::{RefCell, Ref};
use std::rc::Rc;

// For consumed interfaces
use crate::hlcd_infra::console_io_interface::*;
use crate::chess_app::console_ui::board_printer::component::BoardPrinter;
use crate::chess_app::console_ui::command_parser::interface::CommandParserInterface;
use crate::chess_app::console_ui::board_printer::interface::BoardPrinterInterface;
use crate::chess_app::console_ui::turn_cmd_handler::interface::TurnCmdHandlerInterface;
use crate::chess_app::console_ui::game_cmd_handler::interface::GameCmdHandlerInterface;

// Component
// Consumes: CommandParser, TurnCmdHandler, GameCmdHandler, BoardPrinter, ConsoleIO
// Provides: ConsoleApp
// Children: -
pub(super) struct CommandCycle {

    // Dependencies
    console_io: Rc<RefCell<dyn ConsoleIOInterface>>,
    comman_parser: Rc<RefCell<dyn CommandParserInterface>>,
    turn_cmd_handler: Rc<RefCell<dyn TurnCmdHandlerInterface>>,
    game_cmd_handler: Rc<RefCell<dyn GameCmdHandlerInterface>>,
    board_printer: Rc<RefCell<dyn BoardPrinterInterface>>
}

impl CommandCycle {
    pub(super) fn new(
        console_io: &Rc<RefCell<dyn ConsoleIOInterface>>,
        command_parser: &Rc<RefCell<dyn CommandParserInterface>>,
        turn_cmd_handler: &Rc<RefCell<dyn TurnCmdHandlerInterface>>,
        game_cmd_handler: &Rc<RefCell<dyn GameCmdHandlerInterface>>,
        board_printer: &Rc<RefCell<dyn BoardPrinterInterface>>
    ) -> Self {
        Self { 
            console_io: Rc::clone(console_io),
            comman_parser: Rc::clone(command_parser),
            turn_cmd_handler: Rc::clone(turn_cmd_handler),
            game_cmd_handler: Rc::clone(game_cmd_handler),
            board_printer: Rc::clone(board_printer)
        }
    }
}
